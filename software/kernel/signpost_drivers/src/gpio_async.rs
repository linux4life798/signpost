use core::cell::Cell;

use kernel::hil;
use kernel::{AppId, Callback, Driver};
use kernel::returncode::ReturnCode;

use signpost_hil;

pub struct GPIOAsync<'a, Port: signpost_hil::gpio_async::GPIOAsyncPort + 'a> {
    ports: &'a [&'a Port],
    callback: Cell<Option<Callback>>,
}

impl<'a, Port: signpost_hil::gpio_async::GPIOAsyncPort> GPIOAsync<'a, Port> {
    pub fn new(ports: &'a [&'a Port]) -> GPIOAsync<'a, Port> {
        GPIOAsync {
            ports: ports,
            callback: Cell::new(None),
        }
    }

    fn configure_input_pin(&self, port: usize, pin: usize, config: usize) -> ReturnCode {
        let ports = self.ports.as_ref();
        match config {
            0 => {
                ports[port].enable_input(pin, hil::gpio::InputMode::PullUp)
            }

            1 => {
                ports[port].enable_input(pin, hil::gpio::InputMode::PullDown)
            }

            2 => {
                ports[port].enable_input(pin, hil::gpio::InputMode::PullNone)
            }

            _ => ReturnCode::EINVAL,
        }
    }

    // fn configure_interrupt(&self, pin_num: usize, config: usize) -> isize {
    //     let pins = self.pins.as_ref();
    //     match config {
    //         0 => {
    //             pins[pin_num].enable_interrupt(pin_num, InterruptMode::Change);
    //             0
    //         }

    //         1 => {
    //             pins[pin_num].enable_interrupt(pin_num, InterruptMode::RisingEdge);
    //             0
    //         }

    //         2 => {
    //             pins[pin_num].enable_interrupt(pin_num, InterruptMode::FallingEdge);
    //             0
    //         }

    //         _ => -1,
    //     }
    // }
}

impl<'a, Port: signpost_hil::gpio_async::GPIOAsyncPort> signpost_hil::gpio_async::Client for GPIOAsync<'a, Port> {
    fn fired(&self, port_pin_num: usize) {
        self.callback.get().map(|mut cb|
            cb.schedule(1, port_pin_num, 0)
        );
    }

    fn done(&self, value: usize) {
        self.callback.get().map(|mut cb|
            cb.schedule(0, value, 0)
        );
    }
}

impl<'a, Port: signpost_hil::gpio_async::GPIOAsyncPort> Driver for GPIOAsync<'a, Port> {
    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        match subscribe_num {
            0 => {
                self.callback.set(Some(callback));
                ReturnCode::SUCCESS
            }

            // default
            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn command(&self, command_num: usize, data: usize, _: AppId) -> ReturnCode {
        let port = data & 0xFF;
        let pin = (data >> 8) & 0xFF;
        let ports = self.ports.as_ref();

        match command_num {
            // enable output
            0 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].enable_output(pin)
                }
            }

            // set pin
            1 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].set(pin)
                }
            }

            // clear pin
            2 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].clear(pin)
                }
            }

            // toggle pin
            3 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].toggle(pin)
                }
            }

            // enable and configure input
            4 => {
                // XXX: this is clunky
                // data == ((pin_config << 8) | pin)
                // this allows two values to be passed into a command interface
                let pin_num = pin & 0xFF;
                let pin_config = (pin >> 8) & 0xFF;
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    self.configure_input_pin(port, pin_num, pin_config)
                }
            }

            // read input
            5 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].read(pin)
                }
            }

            // enable and configure interrupts on pin, also sets pin as input
            // (no affect or reliance on registered callback)
            6 => {
                // // TODO(brghena): this is clunky
                // // data == ((irq_config << 16) | (pin_config << 8) | pin)
                // // this allows three values to be passed into a command interface
                // let pin_num = data & 0xFF;
                // let pin_config = (data >> 8) & 0xFF;
                // let irq_config = (data >> 16) & 0xFF;
                // if pin_num >= ports.len() {
                //     -1
                // } else {
                //     let mut err_code = self.configure_input_pin(pin_num, pin_config);
                //     if err_code == 0 {
                //         err_code = self.configure_interrupt(pin_num, irq_config);
                //     }
                //     err_code
                // }
                ReturnCode::SUCCESS
            }

            // disable interrupts on pin, also disables pin
            // (no affect or reliance on registered callback)
            7 => {
                // if data >= ports.len() {
                //     -1
                // } else {
                //     ports[data].disable_interrupt();
                //     ports[data].disable();
                //     0
                // }
                ReturnCode::SUCCESS
            }

            // disable pin
            8 => {
                if port >= ports.len() {
                    ReturnCode::EINVAL
                } else {
                    ports[port].disable(pin)
                }
            }

            // default
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}

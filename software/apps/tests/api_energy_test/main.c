#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

#include <timer.h>
#include <tock.h>

#include "signpost_api.h"

static const uint8_t random_i2c_address = 0x40;

int main (void) {
  printf("\n\n[Test] API: Energy\n");

  int rc;

  do {
    rc = signpost_initialization_module_init(
        random_i2c_address,
        SIGNPOST_INITIALIZATION_NO_APIS);
    if (rc < 0) {
      printf(" - Error initializing module (code %d). Sleeping 5s.\n", rc);
      delay_ms(5000);
    }
  } while (rc < 0);

  signpost_energy_information_t info;

  while (true) {
    printf("\nQuery Energy\n");
    printf(  "============\n\n");
    rc = signpost_energy_query(&info);
    if (rc < SUCCESS) {
      printf("Error querying energy: %d\n\n", rc);
    } else {
      printf("Energy Query Result:\n");
      printf("      24h limit: %-4lu mJ\n", info.energy_limit_24h_mJ);
      printf("       24h used: %-4lu mJ\n", info.energy_used_24h_mJ);
      printf("      60s limit: %-4u mA\n",  info.current_limit_60s_mA);
      printf("    60s average: %-4u mA\n",  info.current_average_60s_mA);
      printf("  mJ limit warn: %-4u %%\n",  info.energy_limit_warning_threshold);
      printf("  mJ limit crit: %-4u %%\n",  info.energy_limit_critical_threshold);
      printf("\n");
    }

    printf("Sleeping for 5s\n");
    delay_ms(5000);
  }
}

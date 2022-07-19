#include <stdbool.h>
#include <stdint.h>

#define PEEK(address) (*(uint8_t *)(address))
#define POKE(address, value) *(uint8_t *)(address) = (value)

/**
 * This function must be defined on the Rust side and is
 * called for every IRQ trigger.
 */
void called_every_frame();

typedef void (*function_type)(void);

function_type *const KERNAL_IRQ = (function_type *)0x0314;
function_type *const HARDWARE_IRQ = (function_type *)0xfffe;

/**
 * @brief Initialize raster interrupt
 * @param irq_function Function to call on each triggerin event
 * @param triggering_raster_line VIC-II raster line to trigger irq
 * @param irq_address IRQ address to use, e.g. 0x0315 or 0xfffe
 * @param kill_kernal_and_basic Set to true to disable KERNAL and BASIC roms
 */
void init_raster_irq(function_type irq_function, uint8_t triggering_raster_line,
                     function_type *const irq_address,
                     bool kill_kernal_and_basic) {
  asm volatile(
      "sei\n" // disable maskable IRQs
      "lda #$7f\n"
      "sta $dc0d\n" // disable timer interrupts which can be generated by the
                    // two CIA chips
      "sta $dd0d\n" // the kernal uses such an interrupt to flash the cursor and
                    // scan the keyboard, so we better stop it.
      "lda $dc0d\n" // by reading this two registers we negate any pending CIA
                    // irqs.
      "lda $dd0d\n" // if we don't do this, a pending CIA irq might occur after
                    // we finish setting up our irq. we don't want that to
                    // happen.
      ::
          : "a");

  POKE(0xd01a, 0x01); // tell VICII to generate a raster interrupt
  POKE(0xd012, triggering_raster_line); // raster line to trigger irq

  // as there are more than 256 rasterlines, the topmost bit of $d011 serves
  // as the 9th bit for the rasterline we want our irq to be triggered. here
  // we simply set up a character screen, leaving the topmost bit 0.
  POKE(0xd011, 0x1b);

  if (kill_kernal_and_basic) {
    POKE(0x01, 0x35);
  }

  *irq_address = irq_function; // set interrupt vectors,

  // clear interrupt flag, allowing the CPU to respond to interrupt requests
  asm volatile("cli");
}

/**
 * Wrapper for the imported rust function `called_every_frame()`.
 */
__attribute__((interrupt)) void irq_wrapper(void) {
  called_every_frame();
  // acknowledge the interrupt by clearing the VIC's interrupt flag
  asm volatile("lsr $d019");
}

/**
 * Trigger hardware raster IRQ to the rust defined `called_every_frame()`
 */
void hardware_raster_irq_c(uint8_t triggering_raster_line) {
  init_raster_irq(&irq_wrapper, triggering_raster_line, HARDWARE_IRQ, true);
}

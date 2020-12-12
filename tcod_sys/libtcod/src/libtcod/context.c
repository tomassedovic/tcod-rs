/* BSD 3-Clause License
 *
 * Copyright © 2008-2020, Jice and the libtcod contributors.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#include "context.h"

#include "stdlib.h"
#include "math.h"

struct TCOD_Context* TCOD_context_new_(void)
{
  struct TCOD_Context* renderer = calloc(sizeof(*renderer), 1);
  return renderer;
}

void TCOD_context_delete(struct TCOD_Context* renderer)
{
  if (!renderer) { return; }
  if (renderer->destructor_) { renderer->destructor_(renderer); }
  free(renderer);
}
TCOD_Error TCOD_context_present(
    struct TCOD_Context* context,
    const struct TCOD_Console* console,
    const struct TCOD_ViewportOptions* viewport)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  if (!console) {
    TCOD_set_errorv("Console must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  return context->present_(context, console, viewport);
}
TCOD_Error TCOD_context_screen_pixel_to_tile_d(
    struct TCOD_Context* context, double* x, double* y)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  context->pixel_to_tile_(context, x, y);
  return TCOD_E_OK;
}
TCOD_Error TCOD_context_screen_pixel_to_tile_i(
    struct TCOD_Context* context, int* x, int* y)
{
  double xd = x ? (double)(*x) : 0;
  double yd = y ? (double)(*y) : 0;
  TCOD_Error err = TCOD_context_screen_pixel_to_tile_d(context, &xd, &yd);
  if (x) { *x = (int)floor(xd); }
  if (y) { *y = (int)floor(yd); }
  return err;
}
TCOD_PUBLIC TCOD_Error TCOD_context_save_screenshot(
    struct TCOD_Context* context, const char* filename)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  return context->save_screenshot_(context, filename);
}
TCOD_PUBLIC struct SDL_Window* TCOD_context_get_sdl_window(
    struct TCOD_Context* context)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return NULL;
  }
  return context->get_sdl_window_(context);
}
TCOD_PUBLIC struct SDL_Renderer* TCOD_context_get_sdl_renderer(
    struct TCOD_Context* context)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return NULL;
  }
  return context->get_sdl_renderer_(context);
}
TCOD_Error TCOD_context_change_tileset(
    struct TCOD_Context* context, TCOD_Tileset* tileset)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  return context->set_tileset(context, tileset);
}
int TCOD_context_get_renderer_type(struct TCOD_Context* context)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  return context->type;
}
TCOD_Error TCOD_context_recommended_console_size(
    struct TCOD_Context* context, int* columns, int* rows)
{
  if (!context) {
    TCOD_set_errorv("Context must not be NULL.");
    return TCOD_E_INVALID_ARGUMENT;
  }
  if (!context->cb_recommended_console_size_) {
    TCOD_set_errorv("Context is missing configuration..");
    return TCOD_E_ERROR;
  }
  return context->cb_recommended_console_size_(context, columns, rows);
}

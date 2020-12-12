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
#ifndef WRAPPERS_H
#define WRAPPERS_H

#include "portability.h"
#include "console_types.h"
#include "console_printing.h"
#include "image.h"
#include "mouse.h"
#include "parser.h"

#ifdef __cplusplus
extern "C" {
#endif
/* wrappers to ease other languages integration */
typedef unsigned int colornum_t;

/* color module */
TCODLIB_API bool TCOD_color_equals_wrapper (colornum_t c1, colornum_t c2);
TCODLIB_API colornum_t TCOD_color_add_wrapper (colornum_t c1,
						 colornum_t c2);
TCODLIB_API colornum_t TCOD_color_subtract_wrapper (colornum_t c1,
						 colornum_t c2);
TCODLIB_API colornum_t TCOD_color_multiply_wrapper (colornum_t c1,
						      colornum_t c2);
TCODLIB_API colornum_t TCOD_color_multiply_scalar_wrapper (colornum_t c1,
							     float value);
TCODLIB_API colornum_t TCOD_color_lerp_wrapper(colornum_t c1,
						 colornum_t c2, float coef);
TCODLIB_API void TCOD_color_get_HSV_wrapper(colornum_t c,float * h,
					    float * s, float * v);
TCODLIB_API float TCOD_color_get_hue_wrapper (colornum_t c);
TCODLIB_API float TCOD_color_get_saturation_wrapper (colornum_t c);
TCODLIB_API float TCOD_color_get_value_wrapper(colornum_t c);

/* console module */
/* TCODLIB_API void TCOD_console_set_custom_font_wrapper(const char *fontFile,
                        int char_width, int char_height, int nb_char_horiz,
                        int nb_char_vertic, bool chars_by_row,
                        colornum_t key_color); */

TCODLIB_API void TCOD_console_set_default_background_wrapper(TCOD_console_t con,
						   colornum_t col);
TCODLIB_API void TCOD_console_set_default_foreground_wrapper(TCOD_console_t con,
						   colornum_t col);
TCODLIB_API colornum_t TCOD_console_get_default_background_wrapper(TCOD_console_t con);
TCODLIB_API colornum_t TCOD_console_get_default_foreground_wrapper(TCOD_console_t con);
TCODLIB_API colornum_t TCOD_console_get_char_background_wrapper(TCOD_console_t con,
						       int x, int y);
TCODLIB_API void TCOD_console_set_char_background_wrapper(TCOD_console_t con,int x, int y,
                                      colornum_t col,
                                      TCOD_bkgnd_flag_t flag);
TCODLIB_API colornum_t TCOD_console_get_char_foreground_wrapper (TCOD_console_t con,
                                              int x, int y);
TCODLIB_API void TCOD_console_set_char_foreground_wrapper(TCOD_console_t con,int x, int y,
                                      colornum_t col);
TCODLIB_API void TCOD_console_put_char_ex_wrapper(TCOD_console_t con, int x,
	int y, int c, colornum_t fore, colornum_t back);
TCODLIB_API void TCOD_console_set_fade_wrapper(uint8_t val, colornum_t fade);
TCODLIB_API colornum_t TCOD_console_get_fading_color_wrapper(void);
TCODLIB_API void TCOD_console_set_color_control_wrapper(TCOD_colctrl_t con,
						colornum_t fore,
						colornum_t back);
TCODLIB_API bool TCOD_console_check_for_keypress_wrapper(TCOD_key_t *holder,
							 int flags);
TCODLIB_API void TCOD_console_wait_for_keypress_wrapper(TCOD_key_t *holder,
							bool flush);
TCODLIB_API void TCOD_console_fill_background(TCOD_console_t con, int *r, int *g, int *b);
TCODLIB_API void TCOD_console_fill_foreground(TCOD_console_t con, int *r, int *g, int *b);
TCODLIB_API void TCOD_console_fill_char(TCOD_console_t con, int *arr);

TCODLIB_API void TCOD_console_double_hline(TCOD_console_t con,int x,int y, int l,
					   TCOD_bkgnd_flag_t flag);
TCODLIB_API void TCOD_console_double_vline(TCOD_console_t con,int x,int y,
					   int l, TCOD_bkgnd_flag_t flag);
TCODLIB_API void TCOD_console_print_double_frame(TCOD_console_t con,int x,int y,
						 int w,int h, bool empty,
						 TCOD_bkgnd_flag_t flag,
						 const char *fmt, ...);

TCODLIB_API char *TCOD_console_print_return_string(TCOD_console_t con,int x,
						   int y, int rw, int rh,
						   TCOD_bkgnd_flag_t flag,
						   TCOD_alignment_t align, char *msg,
						   bool can_split,
						   bool count_only);
TCODLIB_API void TCOD_console_set_key_color_wrapper (TCOD_console_t con, colornum_t c);

/* image module */

TCODLIB_API void TCOD_image_clear_wrapper(TCOD_image_t image,
					  colornum_t color);
TCODLIB_API colornum_t TCOD_image_get_pixel_wrapper(TCOD_image_t image,
						      int x, int y);
TCODLIB_API colornum_t TCOD_image_get_mipmap_pixel_wrapper(TCOD_image_t image,
				 float x0,float y0, float x1, float y1);
TCODLIB_API void TCOD_image_put_pixel_wrapper(TCOD_image_t image,int x, int y,
				      colornum_t col);
TCODLIB_API void TCOD_image_set_key_color_wrapper(TCOD_image_t image,
					  colornum_t key_color);

/* mouse module */
TCODLIB_API void TCOD_mouse_get_status_wrapper(TCOD_mouse_t *holder);

/* parser module */
TCODLIB_API colornum_t TCOD_parser_get_color_property_wrapper(TCOD_parser_t parser, const char *name);

/* namegen module */
TCODLIB_API int TCOD_namegen_get_nb_sets_wrapper(void);
TCODLIB_API void TCOD_namegen_get_sets_wrapper(char **sets);

/* sys module */
TCODLIB_API int TCOD_sys_get_current_resolution_x(void);
TCODLIB_API int TCOD_sys_get_current_resolution_y(void);
#ifdef __cplusplus
}
#endif
#endif	/* WRAPPERS_H */

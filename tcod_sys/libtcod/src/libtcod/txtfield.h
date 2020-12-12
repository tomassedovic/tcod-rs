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
#ifndef _TCOD_TEXT_H_
#define _TCOD_TEXT_H_

#include "portability.h"

#include "color.h"
#include "console_types.h"

#ifdef __cplusplus
extern "C" {
#endif
struct TCOD_Text;
typedef struct TCOD_Text *TCOD_text_t;

TCODLIB_API TCOD_text_t TCOD_text_init(int x, int y, int w, int h, int max_chars);
TCODLIB_API TCOD_text_t TCOD_text_init2(int w, int h, int max_chars);
TCODLIB_API void TCOD_text_set_pos(TCOD_text_t txt, int x, int y);
TCODLIB_API void TCOD_text_set_properties(TCOD_text_t txt, int cursor_char, int blink_interval, const char * prompt, int tab_size);
TCODLIB_API void TCOD_text_set_colors(TCOD_text_t txt, TCOD_color_t fore, TCOD_color_t back, float back_transparency);
TCODLIB_API bool TCOD_text_update(TCOD_text_t txt, TCOD_key_t key);
TCODLIB_API void TCOD_text_render(TCOD_text_t txt, TCOD_console_t con);
TCODLIB_API const char * TCOD_text_get(TCOD_text_t txt);
TCODLIB_API void TCOD_text_reset(TCOD_text_t txt);
TCODLIB_API void TCOD_text_delete(TCOD_text_t txt);
#ifdef __cplusplus
}
#endif

#endif /* _TCOD_TEXT_H_ */

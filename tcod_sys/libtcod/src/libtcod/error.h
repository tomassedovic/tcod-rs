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
#ifndef LIBTCOD_ENGINE_ERROR_H_
#define LIBTCOD_ENGINE_ERROR_H_

#ifdef __cplusplus
#include <exception>
#include <string>
#include <stdexcept>
#endif // __cplusplus

#include "config.h"
#include "version.h"
/**
 *  An enum of libtcod error codes.
 *
 *  On values other than `TCOD_E_OK` you can use `TCOD_get_error()` to learn
 *  more information.
 *  \rst
 *  .. versionadded:: 1.16
 *  \endrst
 */
typedef enum TCOD_Error {
  /**
   *  The function completed successfully without issues.
   *
   *  A function is successful when `(err >= 0)`.  Positive values may be used
   *  for warnings, or for other outputs.
   */
  TCOD_E_OK = 0,
  /**
   *  The error code for generic runtime errors.
   *
   *  The returned code my be changed in the future to something more specific.
   *  Use `(err < 0)` to check if the value is an error.
   */
  TCOD_E_ERROR = -1,
  /**
   *  The function failed because a given input argument was invalid.
   */
  TCOD_E_INVALID_ARGUMENT = -2,
  /**
   *  The function failed because it was unable to allocate enough memory.
   */
  TCOD_E_OUT_OF_MEMORY = -3,
  /**
   *  The function completed, but a minor issue was detected.
   */
  TCOD_E_WARN = 1,
} TCOD_Error;
#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
/**
 *  Return the last error message.  If there is no error then the string will
 *  have a length of zero.
 *
 *  The error state is thread specific.
 *  \rst
 *  .. versionadded:: 1.12
 *  \endrst
 */
TCOD_NODISCARD
TCODLIB_API const char* TCOD_get_error(void);
/**
 *  Set an error message and return TCOD_E_ERROR.
 *  \rst
 *  .. versionadded:: 1.12
 *  \endrst
 */
TCODLIB_API TCOD_Error TCOD_set_error(const char* msg);
/**
 *  Set an error message and return TCOD_E_ERROR.
 *  \rst
 *  .. versionadded:: 1.16
 *  \endrst
 */
TCODLIB_FORMAT(1, 2)
TCODLIB_API TCOD_Error TCOD_set_errorf(const char* fmt, ...);
/**
 *  Clear a current existing error message.
 *  \rst
 *  .. versionadded:: 1.16
 *  \endrst
 */
TCODLIB_API void TCOD_clear_error(void);
/**
 *  Set an error with version, file, and line info added to the output.
 *
 *  Used internally.
 */
#define TCOD_set_errorv(msg) \
    TCOD_set_errorf("%s:%i\n%s", \
                    TCOD_STRVERSIONNAME " " __FILE__, __LINE__, (msg))
/**
 *  Format an error with version, file, and line info added to the output.
 *
 *  Used internally.
 */
#define TCOD_set_errorvf(fmt, ...) \
    TCOD_set_errorf("%s:%i\n" fmt, \
                    TCOD_STRVERSIONNAME " " __FILE__, __LINE__, __VA_ARGS__)
#ifdef __cplusplus
} // extern "C"
namespace tcod {
/**
 *  Set an error message and return a relevant error code, usually -1.
 *
 *  Used internally.
 */
inline TCOD_Error set_error(const std::string& msg)
{
  return TCOD_set_errorv(msg.c_str());
}
inline TCOD_Error set_error(const std::exception& e)
{
  return TCOD_set_errorv(e.what());
}
/**
 *  Check and throw error messages.
 *
 *  Used internally.
 */
inline int check_throw_error(int error)
{
  if (error >= 0) { return error; }
  switch (error) {
    case TCOD_E_ERROR:
    default:
      throw std::runtime_error(TCOD_get_error());
      break;
    case TCOD_E_INVALID_ARGUMENT:
      throw std::invalid_argument(TCOD_get_error());
      break;
  }
}
inline TCOD_Error check_throw_error(TCOD_Error error) {
  return static_cast<TCOD_Error>(check_throw_error(static_cast<int>(error)));
}
} // namespace tcod
#endif // __cplusplus
#endif // LIBTCOD_ENGINE_ERROR_H_

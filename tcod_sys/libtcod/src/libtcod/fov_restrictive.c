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
/*
* Mingos' Restrictive Precise Angle Shadowcasting (MRPAS) v1.2
*/
#include "fov.h"

#include <stdlib.h> /* for NULL in VS */

#include "libtcod_int.h"
#include "utility.h"

static void compute_quadrant (
    struct TCOD_Map *m,
    int player_x,
    int player_y,
    int max_radius,
    bool light_walls,
    int dx,
    int dy,
    double*__restrict start_angle,
    double*__restrict end_angle)
{
	/* octant: vertical edge */
	{
		int iteration = 1; /* iteration of the algo for this octant */
		bool done = false;
		int total_obstacles = 0;
		int obstacles_in_last_line = 0;
		double min_angle = 0.0;
		int x;
		int y;

		/* do while there are unblocked slopes left and the algo is within the map's boundaries
		   scan progressive lines/columns from the PC outwards */
		y = player_y+dy; /* the outer slope's coordinates (first processed line) */
		if (y < 0 || y >= m->height) {
			done = true;
		}
		while (!done) {
			/* process cells in the line */
			double slopes_per_cell = 1.0 / (double)(iteration);
			double half_slopes = slopes_per_cell * 0.5;
			int processed_cell = (int)((min_angle + half_slopes) / slopes_per_cell);
			int minx = MAX(0, player_x - iteration);
			int maxx = MIN(m->width - 1, player_x + iteration);
			done = true;
			for (x = player_x + (processed_cell * dx); x >= minx && x <= maxx; x+=dx) {
				int c = x + (y * m->width);
				/* calculate slopes per cell */
				bool visible = true;
				bool extended = false;
				double centre_slope = (double)processed_cell * slopes_per_cell;
				double start_slope = centre_slope - half_slopes;
				double end_slope = centre_slope + half_slopes;
				if (obstacles_in_last_line > 0) {
					if (
						!(
							m->cells[c-(m->width * dy)].fov &&
							m->cells[c-(m->width * dy)].transparent
						) &&
						!(
							m->cells[c-(m->width * dy) - dx].fov &&
							m->cells[c-(m->width * dy) - dx].transparent
						)
					) {
						visible = false;
					} else {
						int idx;
						for (idx = 0; idx < obstacles_in_last_line && visible; ++idx) {
							if (
								start_slope <= end_angle[idx] &&
								end_slope >= start_angle[idx]
							) {
								if (m->cells[c].transparent) {
									if (
										centre_slope > start_angle[idx] &&
										centre_slope < end_angle[idx]
									) {
										visible = false;
									}
								} else {
									if (
										start_slope >= start_angle[idx] &&
										end_slope <= end_angle[idx]
									) {
										visible = false;
									} else {
										start_angle[idx] = MIN(start_angle[idx], start_slope);
										end_angle[idx] = MAX(end_angle[idx], end_slope);
										extended = true;
									}
								}
							}
						}
					}
				}
				if (visible) {
					done = false;
					m->cells[c].fov = true;
					/* if the cell is opaque, block the adjacent slopes */
					if (!m->cells[c].transparent) {
						if (min_angle >= start_slope) {
							min_angle = end_slope;
							/* if min_angle is applied to the last cell in line, nothing more
							   needs to be checked. */
							if (processed_cell == iteration) {
								done = true;
							}
						} else if (!extended) {
							start_angle[total_obstacles] = start_slope;
							end_angle[total_obstacles++] = end_slope;
						}
						if (!light_walls) {
							m->cells[c].fov = false;
						}
					}
				}
				processed_cell++;
			}
			if (iteration == max_radius) {
				done = true;
			}
			iteration++;
			obstacles_in_last_line = total_obstacles;
			y += dy;
			if (y < 0 || y >= m->height) {
				done = true;
			}
		}
	}

	/* octant: horizontal edge */
	{
		int iteration = 1; /* iteration of the algo for this octant */
		bool done = false;
		int total_obstacles = 0;
		int obstacles_in_last_line = 0;
		double min_angle = 0.0;
		int x;
		int y;

		/* do while there are unblocked slopes left and the algo is within the map's boundaries
		   scan progressive lines/columns from the PC outwards */
		x = player_x+dx; /*the outer slope's coordinates (first processed line) */
		if (x < 0 || x >= m->width) {
			done = true;
		}
		while (!done) {
			/* process cells in the line */
			double slopes_per_cell = 1.0 / (double)(iteration);
			double half_slopes = slopes_per_cell * 0.5;
			int processed_cell = (int)((min_angle + half_slopes) / slopes_per_cell);
			int miny = MAX(0, player_y - iteration);
			int maxy = MIN(m->height - 1, player_y + iteration);
			done = true;
			for (y = player_y + (processed_cell * dy); y >= miny && y <= maxy; y += dy) {
				int c = x + (y * m->width);
				/* calculate slopes per cell */
				bool visible = true;
				bool extended = false;
				double centre_slope = (double)processed_cell * slopes_per_cell;
				double start_slope = centre_slope - half_slopes;
				double end_slope = centre_slope + half_slopes;
				if (obstacles_in_last_line > 0) {
					if (
						!(
							m->cells[c-dx].fov &&
							m->cells[c-dx].transparent
						) &&
						!(
							m->cells[c-(m->width * dy) - dx].fov &&
							m->cells[c-(m->width * dy) - dx].transparent
						)
					) {
						visible = false;
					} else {
						int idx;
						for (idx = 0; idx < obstacles_in_last_line && visible; ++idx) {
							if (
								start_slope <= end_angle[idx] &&
								end_slope >= start_angle[idx]
							) {
								if (m->cells[c].transparent) {
									if (
										centre_slope > start_angle[idx] &&
										centre_slope < end_angle[idx]
									) {
										visible = false;
									}
								} else {
									if (
										start_slope >= start_angle[idx] &&
										end_slope <= end_angle[idx]
									) {
										visible = false;
									} else {
										start_angle[idx] = MIN(start_angle[idx], start_slope);
										end_angle[idx] = MAX(end_angle[idx], end_slope);
										extended = true;
									}
								}
								++idx;
							}
						}
					}
				}
				if (visible) {
					done = false;
					m->cells[c].fov = true;
					/* if the cell is opaque, block the adjacent slopes */
					if (!m->cells[c].transparent) {
						if (min_angle >= start_slope) {
						  min_angle = end_slope;
							/* if min_angle is applied to the last cell in line, nothing more
							   needs to be checked. */
							if (processed_cell == iteration) {
								done = true;
							}
						} else if (!extended) {
							start_angle[total_obstacles] = start_slope;
							end_angle[total_obstacles++] = end_slope;
						}
						if (!light_walls) {
							m->cells[c].fov = false;
						}
					}
				}
				processed_cell++;
			}
			if (iteration == max_radius) {
				done = true;
			}
			iteration++;
			obstacles_in_last_line = total_obstacles;
			x += dx;
			if (x < 0 || x >= m->width) {
				done = true;
			}
		}
	}
}

void TCOD_map_compute_fov_restrictive_shadowcasting(TCOD_map_t map, int player_x, int player_y, int max_radius, bool light_walls) {
	/* first, zero the FOV map */
	for(int i = 0, e = map->nbcells; i != e; i++) {
		map->cells[i].fov = false;
	}

	/* set PC's position as visible */
	map->cells[player_x + (player_y*map->width)].fov = true;

	/* calculate an approximated (excessive, just in case) maximum number of obstacles per octant */
	const int max_obstacles = map->nbcells / 7;
	double* start_angle = malloc(max_obstacles * sizeof(*start_angle));
	double* end_angle = malloc(max_obstacles * sizeof(*end_angle));

	/* compute the 4 quadrants of the map */
	compute_quadrant(map, player_x, player_y, max_radius, light_walls, 1, 1, start_angle, end_angle);
	compute_quadrant(map, player_x, player_y, max_radius, light_walls, 1, -1, start_angle, end_angle);
	compute_quadrant(map, player_x, player_y, max_radius, light_walls, -1, 1, start_angle, end_angle);
	compute_quadrant(map, player_x, player_y, max_radius, light_walls, -1, -1, start_angle, end_angle);

	free(end_angle);
	free(start_angle);
}

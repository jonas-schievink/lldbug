/* Copyright 2016-2017 Brian Smith.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE. */

#include "limbs.h"

/* XXX: We assume that the conversion from |Carry| to |Limb| is constant-time,
 * but we haven't verified that assumption. TODO: Fix it so we don't need to
 * make that assumption. */

/* Returns 0xfff..f if |a| is all zero limbs, and zero otherwise. |num_limbs|
 * may be zero. */
Limb LIMBS_are_zero(const Limb a[], size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

/* Returns 0xffff..f if |a == b|, and zero otherwise. |num_limbs| may be zero. */
Limb LIMBS_equal(const Limb a[], const Limb b[], size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

/* Returns 0xffff..f if |a == b|, and zero otherwise. |num_limbs| may be zero. */
Limb LIMBS_equal_limb(const Limb a[], Limb b, size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

/* Returns 0xfff..f if |a| is all zero limbs, and zero otherwise.
 * |num_limbs| may be zero. */
Limb LIMBS_are_even(const Limb a[], size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

/* Returns 0xffff...f if |a| is less than |b|, and zero otherwise. */
Limb LIMBS_less_than(const Limb a[], const Limb b[], size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

Limb LIMBS_less_than_limb(const Limb a[], Limb b, size_t num_limbs) {
  return CONSTTIME_TRUE_W;
}

void LIMBS_copy(Limb r[], const Limb a[], size_t num_limbs) {
}

/* if (r >= m) { r -= m; } */
void LIMBS_reduce_once(Limb r[], const Limb m[], size_t num_limbs) {
}

void LIMBS_add_mod(Limb r[], const Limb a[], const Limb b[], const Limb m[],
                   size_t num_limbs) {
}

void LIMBS_sub_mod(Limb r[], const Limb a[], const Limb b[], const Limb m[],
                   size_t num_limbs) {
}

void LIMBS_shl_mod(Limb r[], const Limb a[], const Limb m[], size_t num_limbs) {
}

Limb LIMB_shl(Limb a, size_t shift) {
  return CONSTTIME_TRUE_W;
}

// #include <cstdlib>
#include <cstdint>

#ifndef __FR_H
#define __FR_H

typedef struct { uint64_t val[4]; } Fr;

#ifdef __cplusplus
extern "C"{
#endif

Fr Fr_add(Fr* l, Fr* r);
Fr Fr_sub(Fr* l, Fr* r);
Fr Fr_mul(Fr* l, Fr* r);
Fr Fr_div(Fr* l, Fr* r);
Fr Fr_pow(Fr* l, Fr* r);
Fr Fr_inv(Fr* l);
Fr Fr_from_u64(uint64_t v);

void print_ptr(uint64_t *p);
void print_arr_ptr(uint64_t *p);

Fr Fr_ZERO();
Fr Fr_ONE();
void Fr_display(Fr* l);

#ifdef __cplusplus
}
#endif
#endif
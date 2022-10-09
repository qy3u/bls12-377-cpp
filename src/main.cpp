#include <iostream>
#include <stdio.h>
#include <stdlib.h>

#include "fr.h"

using namespace std;

int main() {
    Fr zero = Fr_ZERO();
    Fr one = Fr_ONE();

    Fr_display(&zero);
    Fr_display(&one);

    Fr zero_plus_one = Fr_add(&zero, &one);
    Fr_display(&zero_plus_one);

    return 0;
}

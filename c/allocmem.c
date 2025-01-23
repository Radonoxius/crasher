#include<stdlib.h>


//Allocates 50 MiB in the heap.
//Memory isnt free'd for malicious reasons.
double *alloc_fifty_mib() {
    return (double *)malloc(sizeof(double) * 128 * 1024 * 50);
}

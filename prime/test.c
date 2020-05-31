#include <stdio.h>
#include <math.h>
#include <time.h>
#include <sys/time.h>
#include <sys/resource.h>

#define true 1
#define false 0

double get_time() {
    struct timeval t;
    struct timezone tzp;
    gettimeofday(&t, &tzp);
    return t.tv_sec + t.tv_usec*1e-6;
}

int isPrime (int n) {
    if (n < 2)      return false;
    if (n == 2)     return true;
    if (n == 3)     return true;
    if (n % 2 == 0) return false;
    if (n % 3 == 0) return false;
    if (n % 1)      return false;
    double sqrtOfN = sqrt(n);
    for (int i = 5; i <= sqrtOfN; i += 6){
        if (n % i == 0) return false;
        if (n % (i + 2) == 0) return false;
    }
    return true;
};

int main(int argc, const char * argv[]) {
    int count = 0;
    double time = get_time();
    for (int i = 0; i < 10000000; i++){
        if (isPrime(i)){
            count++;
        }
    }

    printf("%f\n", get_time() - time);
    return 0;
}

import 'dart:math';

bool isPrime (int n){
    if (n < 2)      return false;
    if (n == 2)     return true;
    if (n == 3)     return true;
    if (n % 2 == 0) return false;
    if (n % 3 == 0) return false;
    if (n % 1 > 0)  return false;
    double sqrtOfN = sqrt(n);
    for (int i = 5; i <= sqrtOfN; i += 6){
        if (n % i == 0) return false;
        if (n % (i + 2) == 0) return false;
    }
    return true;
}

void countPrime() {
    int count = 0;

    for (int i = 0; i < 10000000; i++){
        if (isPrime(i)){
            count++;
        }
    }
}

void main() {
  final time = DateTime.now().millisecondsSinceEpoch;
  countPrime();
  print((DateTime.now().millisecondsSinceEpoch - time)/1000);
}

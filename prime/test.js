"use strict";

var isPrime = function(n){
    //if (n !== parseInt(n,10)) {return false};
    if (n < 2) {return false};
    if (n === 2) {return true};
    if (n === 3) {return true};
    if (n % 2 === 0) {return false};
    if (n % 3 === 0) {return false};
    if (n % 1) {return false};
    var sqrtOfN = Math.sqrt(n);
    for (var i = 5; i <= sqrtOfN; i += 6){
        if (n % i === 0) {return false}
        if (n % (i + 2) === 0) {return false}
    }
    return true;
};

var countPrime = function(){
    let count = 0;
    const time = Date.now();

    for (let i = 1; i < 10000000;i++){
        if (isPrime(i)){
            count++;
        }
    }

    console.log((Date.now() - time) / 1000);
};

countPrime();


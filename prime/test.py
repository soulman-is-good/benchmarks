import math
from datetime import datetime

try:
  rng = xrange
except:
  rng = range

def isPrime (n):
    if n < 2       : return False
    if n == 2      : return True
    if n == 3      : return True
    if n % 2 == 0  : return False
    if n % 3 == 0  : return False
    if n % 1 > 0   : return False
    sqrtOfN = int(math.sqrt(n)) + 1
    for i in rng (5, sqrtOfN, 6):
        if n % i == 0       : return False
        if n % (i + 2) == 0 : return False
    return True

count = 0
start = datetime.now()
for i in rng(10000000) :
    if isPrime(i) :
        count+=1

print((datetime.now() - start).total_seconds())
<?php
function milliseconds() {
  $mt = explode(' ', microtime());
  return ((int)$mt[1]) * 1000 + ((int)round($mt[0] * 1000));
}

function isPrime($n) {
  if ($n < 2)      return false;
  if ($n == 2)     return true;
  if ($n == 3)     return true;
  if ($n % 2 == 0) return false;
  if ($n % 3 == 0) return false;
  if ($n % 1 > 0)  return false;

  $sqrtOfN = sqrt($n);

  for ($i = 5; $i <= $sqrtOfN; $i += 6){
      if ($n % $i == 0) return false;
      if ($n % ($i + 2) == 0) return false;
  }
  return true;
}

$count = 0;
$time = milliseconds();

for ($i = 1; $i < 10000000; $i++){
    if (isPrime($i)){
        $count++;
    }
}

printf("%f\n", (milliseconds() - $time) / 1000);

?>
# aleo_python

# usage

```
import aleo_python

try:
  signature = aleo_python.sign_message("APrivateKey1zkpHkzXqGeMrDtTzfze8Uq2ArMy3UJSQMQhfE2Tdv7HBJYE", "test")
  print(signature)

except ValueError as err_message:
  print(err_message)

try:
  verify_signature = aleo_python.verify_message("aleo1m9wl26xlnz854v9hs9xlyxppy7c39039mhttpe06vhzkcsatuv9qdamckq", "test", "sign1rgx7wh54r4pllym6dlq37ykpg4uhpy4p8cdex9tpvfmmua72ssp0f3t7zcqz72vryzd3k40qgcnvprm4h52ql0gsleu9xjj5xgw35qgsdfl7wxyzvkr3zn2tw7rf3vmy800dv5xs2klzu6x36hvakf5zpfvlan6gx66pstcfufpslny709u9aq6ch2h87ayjn9wtaum707ms79p6enk")
  print(verify_signature)

except ValueError as err_message:
  print(err_message)
```

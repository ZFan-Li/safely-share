# Share data with others safely

This is a interesting code snippet that can split a file to several
pieces and encipher them to bytes which cannot be deciphered
separately. In fact, it is impossible for anyone to get even one bit
of information about the original from the ciphers except they get all
pieces of them.

## Example

Yeager wrote a extremely horrible code. It is so dangerous that people
decided to split the right of executing it to Alice, Bob and Carol, so
the source code can be compiled and executed only if all them reach an
agreement that it should be ran.

```perl
$ cat <<EOF | safely share --with Alice Bob Carol
> #include <stdio.h>
>
> int man(char argv[]) {
>   int repeat = argv[0];
>   printf("the Rumbling!" * repeat);
> }
>
EOF
$ ls -a
.  ..  Alice  Bob  Carol
$ xxd Alice
00000000: 4626 8b8f 6cfb f8bc d900 1410 2f2f 4b67  F&..l.......//Kg
00000010: 6b9b 1fa4 a7f3 f655 e76c ac96 268e 9869  k......U.l..&..i
00000020: 74eb 5833 91ca 54ea 0698 ccf9 d8dd d981  t.X3..T.........
00000030: a0ec eecd e7cd 70d8 f4fa 392c 5eeb 5dff  ......p...9,^.].
00000040: 6fc1 e204 5d1c 7865 6fae 5364 d785 2ab5  o...].xeo.Sd..*.
00000050: 7a4f cf44 864e be24 6f6a 6df9 b84a ab25  zO.D.N.$ojm..J.%
00000060: 67d0 a014 390c 9eb9 4688                 g...9...F.
$ safely gather *
#include <stdio.h>

int man(char argv[]) {
  int repeat = argv[0];
  printf("the Rumbling!" * repeat);
}
```

### License: GPLv3

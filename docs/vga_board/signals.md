
## Horizontal signals

### Common

A = (!1 && !4) && (64 && 128)

B = 2 && !8

### Trigger values

200 = A !&& ((!2 && 8) && (!16 && !32))

210 = A !&& (B && (16 && !32))

242 = A !&& (B && (16 && 32))

264 = 8 !&& 256


### Outputs


|   S   |   R   |    Q    |   !Q   |
|-------|-------|---------|--------|
|  200  |  264  | !VBLANK | VBLANK |
|  210  |  242  |  VSYNC  |   NC   |

## Vertical signals

### Common

A = (!2 && 16) && (64 && 512)

B = 8 && !32

### Trigger values

600 = A !&& (B && (!1 && !4))

601 = A !&& (B && (1 && !4))

605 = A !&& (B && (1 && 4))

628 = A !&& (4 && 32)


### Outputs


|   S   |   R   |    Q    |   !Q   |
|-------|-------|---------|--------|
|  600  |  628  | !HBLANK | HBLANK |
|  601  |  605  |  HSYNC  |   NC   |


## Connectors pinout

### (VGA signals) - (Framebuffers) - (Palette mapper) connector (IDC 40pin)

```
         ------------
         |          |
  VSync  |  40  39  | Clock
  HSync  |  38  37  |
 !VBlank |  36  35  | VBlank
 !HBlank |  34  33  | HBlank
    Y0   |  32  31  |   X0
    Y1   |  30  29  |   X1
    Y2   |  28  27  |   X2
    Y3   |  26  25  |   X3
    Y4   |  24  23  |   X4
    Y5   |  22  21      X5
    Y6   |  20  19      X6
    Y7   |  18  17  |   X7
         |  16  15  |
         |  14  13  |
         |  12  11  |
         |  10   9  |
    D7   |   8   7  |   D6
    D5   |   6   5  |   D4
    D3   |   4   3  |   D2
    D1   |   2   1  |   D0
         |          |
         ------------
```

### VGA port (IDC 16pin)

```
         ------------
         |          |
         |  16  15  |
   VSync |  14  13  | HSync
         |  12  11  |
     GND |  10   9
     GND |   8   7    GND
     GND |   6   5  |
         |   4   3  | Blue
   Green |   2   1  | Red
         |          |
         ------------
```

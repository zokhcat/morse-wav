# Morse WAV
alan turing used electromechanical machines to decipher morse code, I used rust lmfao.

note: as of now this works on linux.

## Implementation
- [x] Get all the sets for each characters and numbers
- [x] Integrate strings -> morse code + sound
- [x] Integrate morse code -> sound
- [x] Integrate clap for CLI

## Requirements
Download the morse_wav binary from the [releases](https://github.com/zokhcat/morse-wav/releases/tag/v0.0.1) page.

###### Make it executable
```sh
chmod +x morse_wav
```

## Usage
<br>
###### Translate a string
<br>
```sh
./morse_wav text "climate change is a hoax, says oil lobby"
```

###### Morse code
<br>
```sh
./morse_wav play ".... . .-.. .-.. ---"
```
# Lystem
Lystem is cross platform command line application to render L-System interpreted as turtle instructins. The program renders a series of images, one for each instruction (or more). It does not directly make a video, for that you can use something like ffmpeg with:
```
ffmpeg -r 60 -i images/out%d.png output.mp4
```

The LSystem generator is made to occupy as little memory as possible, so it can run on limited devices too.

## Usage
You can build the project with `cargo`
```
cargo run --release -- [ARGS]
```

There are two required parameters, the input configuration, and the number of generations to simulate
```
cargo run --release configs/maze.yml 3
```
This will render the images of the 3rd generation of maze.yml, interpreted by the turtle as specified in the file.

Optional parameters are:
* `-l` which sets the program to only render the last frame (when all the turtle instruction have been executed)
* `-s` which sets how many turtle instructions should be executed in each frame (translates to the playback speed, there more instruction you interpret per frame, the faster the video will be)

## Configuration files
The L-System axiom and rules, along with the instructions to interpret the characters as turtle commands are written in yaml files. You can find many examples in the `configs` directory.

```yml
axiom: F+F+F+F

start_state:
  'color_r': 255
  'color_b': 255
  'step': 1
  'turning_angle': 90

rules:
  'F': FF+F+F+F+FF

commands:
  'F':
    - forward
    - add color_r -1
  '+':
    - clockwise
```

* `axiom` indicates the system axiom, each character is interpreted as a single symbol, only ASCII character are supported
* `start_state` describes the turtle initial state, there are many variables accessible:
  * `color_r`, `color_g`, `color_b` to indicate the r,g,b values of the turtle drawing color, from 0 to 255
  * `step` which sets how far the turtle travels each time it goes forward
  * `turning_angle` turning angle of the turle used by the `clockwise` and `counterclockwise` commands
  * `rotation` the initial rotation of the turtle

* `rules` tells how each symbol evolves
* `commands` says how to interpret the symbol as a turtle command
  * `forward` makes the turtle go foward by `step`
  * `clockwise`/`counterclockwise` makes the turtle turn clockwise/counterclockwise by `turning_angle`
  * `push_stack` pushed the current turtle state on the stack
  * `pop_stack` pops the stack on the current turtle state
  * `add`/`multiply`/`set` are functions which take two arguments, the variable to change and the value to apply. A variable can also be used as a value: `add turning_angle step` adds the value of step to the turning_angle. An integer can also handle operations with a decimal number (`multiply color_r 1.5`), being rounded toward -infinity after the calculations.
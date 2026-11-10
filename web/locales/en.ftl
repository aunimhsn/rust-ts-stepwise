# The texts of the interface in English, the reference language: every other language has
# the same messages. The syntax is Fluent's: https://projectfluent.org/fluent/guide/

app-name = Stepwise
app-tagline = Learn to program step by step.

## The page of step 4b, replaced by the home screen in step 13.

# $version (String): the version of the core, as the WebAssembly module gives it.
core-version = Core { $version }, computed in Rust.
core-load-failed = The core of Stepwise could not load.

## The probe of step 4c: a fake program run at each animation frame, replaced by the level
## screen of step 13 onward.

probe-title = A fake program, run one step at each frame
probe-map = The robot walking around a square on a small map.
probe-run = Run
probe-pause = Pause
probe-reset = Reset
probe-speed = Speed
# $rate (Number): the steps run each second at this speed.
probe-speed-rate =
    { $rate ->
        [one] { $rate } step per second
       *[other] { $rate } steps per second
    }
probe-speed-instant = Instant
# $count (Number): the steps run since the start.
probe-steps =
    { $count ->
        [one] { $count } step
       *[other] { $count } steps
    }
# $rate (Number): the steps run each second at Instant, measured over the last second.
probe-measured = { $rate } steps per second measured

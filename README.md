A simple command-line built tool using Clap. Generates a properly formatted .wav file according to [http://soundfile.sapp.org/doc/WaveFormat/](http://soundfile.sapp.org/doc/WaveFormat/)

# Usage

`cl_wav <PATH> <SAMPLES> <SAMPLE_RATE> <FREQUENCY> <MS_TAPER>` : Writes a wav file to `<PATH>` consisting of `<SAMPLES>` 16-bit samples of a sine wave at `<FREQUENCY>` hz and -6 dBFS (half maximum amplitude) and `<SAMPLE_RATE>` samples/hz, tapering off for `<MS_TAPER>` milliseconds at the end of the file. Hack it into whatever you like!
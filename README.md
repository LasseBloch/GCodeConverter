# GCodeConverter
A small tool to converted the g-code generated by inkscape into something that Klipper will accept

I've been build a plotter, and for its running Klipper (As all the cool kids) for its firmware.
I use Inkscape to convert images to G-code, via Gcodetools. The resulting g-code, is
not something that Klipper will accept.

 So this is my little converter tool, still very much a work in progress.
 
 It will at the moment due the following convertions
 
 * G0X to GX conversion
 * Replace all movements of an axis to another. E.g replace all Z1.0 with Z0

It will read the original g-code file, do the conversion and wirte the converted g-codes to a new file.

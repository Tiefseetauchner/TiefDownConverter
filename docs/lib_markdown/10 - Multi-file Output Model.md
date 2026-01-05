# Multi-file Output Model

Templates can not only produce single files, but in case of CustomPreProcessors conversion, they are able to produce multi-file outputs. When enabled, multi-file output allows the conversion to export one output file for each input file (including body injections).

An example for a use case of multi-file output is a wiki. You may want to provide a printable PDF of your wiki while also creating a statically rendered website from the same inputs. Multi-file output allows for the rendered website to not be a single page. 

Injections for multi-file outputs have a slightly different mental model to other templates. Instead of rendering the injections once, they are rendered *for each output file separately*, meaning the injection can include code that is unique to each output file.

For example, a header injection could include accessing the navigation metadata (see below) to render the current position in a tree. The concrete implementation is left as an exercise to the reader.

Multi-file output is enabled on a per-template basis and is only available to CustomPreProcessors conversions.

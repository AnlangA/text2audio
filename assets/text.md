To show how the different primitive operations behave, we
will measure them using identical benchmarks on each of the
machines. The goal is not to conduct a bake off between vendors
or architectures to show that one processor is better than
another (that is for the market to decide). The benchmarks we
present are meant to show that the issues we discuss are
common to these different architectures and implementations,
and therefore need to be considered when designing a parallel
runtime system that will have to run on these machines or
similar ones. Obviously, one can measure a specific machine and
then design code that works well on it; however, if the code is to
succeed, then it will have a longer life than any specific processor
implementation, and therefore we want it to work on a variety of
current and future hardware. Since we don’t have a time
machine, we can’t be sure what the performance quirks of such
future machines will be; however, by looking at a variety of
current machines, we can see what performance characteristics
are common and, therefore, likely to persist.

def f():  # type: ignore
    ...

class x:  # some comment
    ...

class y:
    ...  # comment

# whitespace doesn't matter (note the next line has a trailing space and tab)
class z:        
    ...

def g():
    # hi
    ...

# FIXME(#8905): Uncomment, leads to unstable formatting
# def h():
#     ...
#     # bye


# Debugger GUI

## why rust wasm?

Because JavaScript is such a maintenance hell, and TypeScript lacks many things, and unifying server and client languages simplifies the code for communication.

However, rust compilation is seriously slow. There really should be a purely interpreted rust.

## pitfalls

Reactive data
Accepting data from the parent sure is nice but it would be even better if updating the data in the parent also updates the view in the child component! For components to automatically react to prop changes, they should accept a signal. Most of the times, you’ll want a &ReadSignal unless you want mutable access to the data in which case you should use a &Signal. This way, updating the signal will automatically update whatever is listening to it, even if it is inside the child component.


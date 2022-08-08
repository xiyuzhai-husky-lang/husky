document.defaultView.addEventListener("resize", (ev) =>
    console.log("defaultView: ", ev)
);
document
    .getElementsByTagName("p")[0]
    .addEventListener("resize", (ev) => console.log("defaultView: ", ev));
let blah = document.getElementById("blah");
blah.addEventListener("keydown", (ev) => consolle.log(ev));

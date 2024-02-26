// this code is stolen directly from view-source:https://www.georgehart.com/virtual-polyhedra/conway_notation.html
// let's be sure to credit him for this reference material when we finish

//--------------------form interface functions---------------------
// Form is at bottom of this page.  Output appears in new "window2"

function viewVRML() { // create vrml object in another window
	if (goodInput()) {
		state("Initializing vrml window...");
		window2 = open("", "secondWindow", "resizable=yes")
		window2.document.open('x-world/x-vrml');
		generateVRML();
		window2.document.close();
	}
}

function viewSource() { // show vrml code in another window
	if (goodInput()) {
		state("Initializing text window...");
		window2 = open("", "secondWindow", "scrollbars=yes")
		window2.document.open('text/plain');
		generateVRML();
		window2.document.close();
	}
}

function generateVRML() { // produce specified vrml object
	inform(""); // clear the info window
	var poly = generatePoly(document.input.notation.value); // create poly.
	outputVRML(poly);
	state("Done with " + poly.name + "  " + data(poly));
}

function goodInput() { // check input format
	var question = document.input.notation.value;
	if (question.search(/([^ktajsgebomdcrpTCOIDPAY0123456789])/) != -1) {
		alert("Undefined character: " + RegExp.lastParen);
		return (false);
	}
	if (question.search(/^([ajsgebomdcrp]|([kt][0-9]*))*([TCOID]$|[PAY][0-9]+$)/) == -1) {
		alert("Ill-formed polyhedron expression: " + question);
		return (false);
	}
	if (question.search(/([0-9]+)$/) != -1 && RegExp.lastParen < 3) {
		alert("At least 3 sides are required, not " + RegExp.lastParen);
		return (false);
	}
	return (true); // found no problems
}

function solidColor() { // read state of radio buttons
	if (document.buttons.color[0].checked == 1)
		return ("1 1 1    # all faces white");
	else
		return (""); // null string means go by number of sides
}

function dualColor() { // read state of radio buttons
	if (document.buttons.color[0].checked == 1)
		return ("0.55 0.35 0.05    # all faces brown");
	else
		return (""); // null string means go by number of sides
}

function compound() { // read state of radio buttons
	//   The program is set up to output compound with dual as well, but this 
	//   requires a careful canonicalization, which is rather slow.  So I have 
	//   commented out this option, until I come up with a faster method.
	//   return (document.buttons.compound[1].checked == 1)
	return (false);
}

function state(x) { // put message in status bar
	window.status = x;
}

function inform(x) { // print info on form
	if (x == null) x = "";
	document.output.inform.value = x.toString();
}

function example(what) { // called from html tag
	document.input.notation.value = what; // sets input text as if typed
	viewVRML();
}

//------------------notation processing functions--------------------

function generatePoly(notation) { // create polyhedron from notation
	var poly = new polyhedron(); // each polyhedron, during construction
	var n = 0; // numeric argument

	state("Analyzing input...");
	var ops = getOps(notation);

	if (ops == globSavedPoly.name) // if same as last time, use stored poly.
		return (globSavedPoly);
	if (ops == globSavedDual.name) // if dual of last time, use stored dual.
		return (dual());
	if (ops.substr(-globSavedPoly.name.length) == globSavedPoly.name) {
		ops = ops.substr(0, ops.length - globSavedPoly.name.length);
		poly = globSavedPoly; // extend previous poly
	} else if (ops.substr(-globSavedDual.name.length) == globSavedDual.name) {
		ops = ops.substr(0, ops.length - globSavedDual.name.length);
		poly = dual(); // extend previous dual
	} else { // start afresh
		if (ops.search(/([0-9]+)$/) != -1) { // get number if present
			n = 1 * RegExp.lastParen;
			ops = ops.slice(0, -RegExp.lastParen.length);
		}
		state("Constructing seed...");
		if (ops.slice(-1) == "T") poly = tetrahedron();
		if (ops.slice(-1) == "O") poly = octahedron();
		if (ops.slice(-1) == "C") poly = cube();
		if (ops.slice(-1) == "I") poly = icosahedron();
		if (ops.slice(-1) == "D") poly = dodecahedron();
		if (ops.slice(-1) == "P") poly = prism(n);
		if (ops.slice(-1) == "A") poly = antiprism(n);
		if (ops.slice(-1) == "Y") poly = pyramid(n);
	}
	for (; ops != "";) { // while loop
		n = 0;
		if (ops.search(/([0-9]+)$/) != -1) { // get number if present
			n = 1 * RegExp.lastParen;
			ops = ops.slice(0, -RegExp.lastParen.length);
			if (n < 3)
				alert("Of course you know that a value of " + n + " makes no sense, but I'll look anyway.");
		}
		if (ops.slice(-1) == "k") poly = kisN(poly, n);
		if (ops.slice(-1) == "a") poly = ambo(poly);
		if (ops.slice(-1) == "g") poly = gyro(poly);
		if (ops.slice(-1) == "p") poly = propellor(poly);
		if (ops.slice(-1) == "d") poly = dual(); // dual already computed
		if (ops.slice(-1) == "c") poly.xyz = canonicalXYZ(poly, 10);
		if (ops.slice(-1) == "r") poly = reflect(poly);
		ops = ops.slice(0, -1); // remove last character
	}
	//   poly.xyz = canonicalXYZ(poly, 5)     // refine final coords of poly and dual
	return (poly);
}

function getOps(question) { //  Convert notation into string of ops
	var ans = question; // Global replacements in notation:
	ans = ans.replace(/P4$/g, "C"); // P4 --> C   (C is prism)
	ans = ans.replace(/A3$/g, "O"); // A3 --> O   (O is antiprism)
	ans = ans.replace(/Y3$/g, "T"); // Y3 --> T   (T is pyramid)
	ans = ans.replace(/e/g, "aa"); // e --> aa   (abbr. for explode)
	ans = ans.replace(/b/g, "ta"); // b --> ta   (abbr. for bevel)
	ans = ans.replace(/o/g, "jj"); // o --> jj   (abbr. for ortho)
	ans = ans.replace(/m/g, "kj"); // m --> kj   (abbr. for meta)
	ans = ans.replace(/t(\d*)/g, "dk$1d"); // t(n) --> dk(n)d  (dual operations)
	ans = ans.replace(/j/g, "dad"); // j --> dad  (dual operations)
	ans = ans.replace(/s/g, "dgd"); // s --> dgd  (dual operations)
	ans = ans.replace(/dd/g, ""); // dd --> null  (order 2)
	ans = ans.replace(/ad/g, "a"); // ad --> a   (a_ = ad_)
	ans = ans.replace(/gd/g, "g"); // gd --> g   (g_ = gd_)
	ans = ans.replace(/aY/g, "A"); // aY --> A   (interesting fact)
	ans = ans.replace(/dT/g, "T"); // dT --> T   (self-dual)
	ans = ans.replace(/gT/g, "D"); // gT --> D   (symm change)
	ans = ans.replace(/aT/g, "O"); // aT --> O   (symm change)
	ans = ans.replace(/dC/g, "O"); // dC --> O   (dual pair)
	ans = ans.replace(/dO/g, "C"); // dO --> C   (dual pair)
	ans = ans.replace(/dI/g, "D"); // dI --> D   (dual pair)
	ans = ans.replace(/dD/g, "I"); // dD --> I   (dual pair)
	ans = ans.replace(/aO/g, "aC"); // aO --> aC  (for uniqueness)
	ans = ans.replace(/aI/g, "aD"); // aI --> aD  (for uniqueness)
	ans = ans.replace(/gO/g, "gC"); // gO --> gC  (for uniqueness)
	ans = ans.replace(/gI/g, "gD"); // gI --> gD  (for uniqueness)
	inform(question + " executed as " + ans);
	return (ans);
}

//------------------------polyhedra functions------------------

// Topology stored as set of "faces."  Each face is list of n 0-based vertex indices
// corresponding to one n-sided face.  Vertices listed clockwise as seen from outside.   
// See cube below for example.  The term "flag" refers to a triple of a face index and 
// two adjacent vertex indices, in clockwise order.

// Two global variables save the most recently constructed polyhedron and its dual
// in case the user asks for the same one again but with different output options.
// These are updated after canonicalization or taking the dual:

var globSavedPoly = new polyhedron(); // global. poly from last canonicalization
var globSavedDual = new polyhedron(); // global. dual from last canonicalization

function polyhedron() { // constructor of initially null polyhedron
	this.face = new Array(); // array of faces.          face.length = # faces
	this.xyz = new Array(); // array of vertex coords.  xyz.length = # of vertices
	this.name = "null polyhedron"
}

function data(poly) { // informative string
	var nEdges = poly.face.length + poly.xyz.length - 2; // E = V + F -2
	return ("(" + poly.face.length + " faces, " + nEdges + " edges, " + poly.xyz.length + " vertices)");
}

//-------------------------primative polyhedra-----------------

function tetrahedron() {
	var ans = new polyhedron();
	ans.name = "T";
	ans.face = new Array(
		new Array(0, 1, 2), new Array(0, 2, 3), new Array(0, 3, 1), new Array(1, 3, 2));
	ans.xyz = new Array(
		new Array(1., 1., 1.), new Array(1., -1., -1.),
		new Array(-1., 1., -1.), new Array(-1., -1., 1.))
	return (ans)
}

function octahedron() {
	var ans = new polyhedron();
	ans.name = "O";
	ans.face = new Array(
		new Array(0, 1, 2), new Array(0, 2, 3), new Array(0, 3, 4), new Array(0, 4, 1),
		new Array(1, 4, 5), new Array(1, 5, 2), new Array(2, 5, 3), new Array(3, 5, 4));
	ans.xyz = new Array(
		new Array(0, 0, 1.414), new Array(1.414, 0, 0), new Array(0, 1.414, 0),
		new Array(-1.414, 0, 0), new Array(0, -1.414, 0), new Array(0, 0, -1.414))
	return (ans)
}

function cube() {
	var ans = new polyhedron();
	ans.name = "C";
	ans.face = new Array(
		new Array(3, 0, 1, 2), new Array(3, 4, 5, 0), new Array(0, 5, 6, 1),
		new Array(1, 6, 7, 2), new Array(2, 7, 4, 3), new Array(5, 4, 7, 6));
	ans.xyz = new Array(
		new Array(0.707, 0.707, 0.707), new Array(-0.707, 0.707, 0.707),
		new Array(-0.707, -0.707, 0.707), new Array(0.707, -0.707, 0.707),
		new Array(0.707, -0.707, -0.707), new Array(0.707, 0.707, -0.707),
		new Array(-0.707, 0.707, -0.707), new Array(-0.707, -0.707, -0.707))
	return (ans)
}

function icosahedron() {
	var ans = new polyhedron();
	ans.name = "I";
	ans.face = new Array(
		new Array(0, 1, 2), new Array(0, 2, 3), new Array(0, 3, 4), new Array(0, 4, 5),
		new Array(0, 5, 1), new Array(1, 5, 7), new Array(1, 7, 6), new Array(1, 6, 2),
		new Array(2, 6, 8), new Array(2, 8, 3), new Array(3, 8, 9), new Array(3, 9, 4),
		new Array(4, 9, 10), new Array(4, 10, 5), new Array(5, 10, 7), new Array(6, 7, 11),
		new Array(6, 11, 8), new Array(7, 10, 11), new Array(8, 11, 9), new Array(9, 11, 10));
	ans.xyz = new Array(
		new Array(0, 0, 1.176), new Array(1.051, 0, 0.526),
		new Array(0.324, 1., 0.525), new Array(-0.851, 0.618, 0.526),
		new Array(-0.851, -0.618, 0.526), new Array(0.325, -1., 0.526),
		new Array(0.851, 0.618, -0.526), new Array(0.851, -0.618, -0.526),
		new Array(-0.325, 1., -0.526), new Array(-1.051, 0, -0.526),
		new Array(-0.325, -1., -0.526), new Array(0, 0, -1.176))
	return (ans)
}

function dodecahedron() {
	var ans = new polyhedron();
	ans.name = "D";
	ans.face = new Array(
		new Array(0, 1, 4, 7, 2), new Array(0, 2, 6, 9, 3), new Array(0, 3, 8, 5, 1),
		new Array(1, 5, 11, 10, 4), new Array(2, 7, 13, 12, 6), new Array(3, 9, 15, 14, 8),
		new Array(4, 10, 16, 13, 7), new Array(5, 8, 14, 17, 11), new Array(6, 12, 18, 15, 9),
		new Array(10, 11, 17, 19, 16), new Array(12, 13, 16, 19, 18), new Array(14, 15, 18, 19, 17));
	ans.xyz = new Array(
		new Array(0, 0, 1.07047), new Array(0.713644, 0, 0.797878),
		new Array(-0.356822, 0.618, 0.797878), new Array(-0.356822, -0.618, 0.797878),
		new Array(0.797878, 0.618034, 0.356822), new Array(0.797878, -0.618, 0.356822),
		new Array(-0.934172, 0.381966, 0.356822), new Array(0.136294, 1., 0.356822),
		new Array(0.136294, -1., 0.356822), new Array(-0.934172, -0.381966, 0.356822),
		new Array(0.934172, 0.381966, -0.356822), new Array(0.934172, -0.381966, -0.356822),
		new Array(-0.797878, 0.618, -0.356822), new Array(-0.136294, 1., -0.356822),
		new Array(-0.136294, -1., -0.356822), new Array(-0.797878, -0.618034, -0.356822),
		new Array(0.356822, 0.618, -0.797878), new Array(0.356822, -0.618, -0.797878),
		new Array(-0.713644, 0, -0.797878), new Array(0, 0, -1.07047))
	return (ans)
}

function prism(n) {
	var theta = 6.283185 / n; // pie angle
	var h = Math.sin(theta / 2); // half-edge
	var ans = new polyhedron();
	ans.name = "P" + n;

	for (var i = 0; i < n; i++) // vertex #'s 0...n-1 around one face
		ans.xyz[ans.xyz.length] = new Array(Math.cos(i * theta), Math.sin(i * theta), h);
	for (var i = 0; i < n; i++) // vertex #'s n...2n-1 around other
		ans.xyz[ans.xyz.length] = new Array(Math.cos(i * theta), Math.sin(i * theta), -h);

	ans.face[ans.face.length] = sequence(n - 1, 0); // top
	ans.face[ans.face.length] = sequence(n, 2 * n - 1); // bottom
	for (var i = 0; i < n; i++) // n square sides:
		ans.face[ans.face.length] = new Array(i, (i + 1) % n, (i + 1) % n + n, i + n);

	ans.xyz = adjustXYZ(ans, 1);
	return (ans);
}

function antiprism(n) {
	var theta = 6.283185 / n; // pie angle
	var h = Math.sqrt(1 - 4 / (4 + 2 * Math.cos(theta / 2) - 2 * Math.cos(theta))); // half-height
	var r = Math.sqrt(1 - h * h); // radius of face circle
	var f = Math.sqrt(h * h + Math.pow(r * Math.cos(theta / 2), 2));
	r = r / f; // correction so edge midpoints (not vertices) on unit sphere
	h = h / f;
	var ans = new polyhedron();
	ans.name = "A" + n;

	for (var i = 0; i < n; i++) // vertex #'s 0...n-1 around one face
		ans.xyz[ans.xyz.length] = new Array(r * Math.cos(i * theta), r * Math.sin(i * theta), h);
	for (var i = 0; i < n; i++) // vertex #'s n...2n-1 around other
		ans.xyz[ans.xyz.length] = new Array(r * Math.cos((i + 0.5) * theta), r * Math.sin((i + 0.5) * theta), -h);

	ans.face[ans.face.length] = sequence(n - 1, 0); // top
	ans.face[ans.face.length] = sequence(n, 2 * n - 1); // bottom
	for (var i = 0; i < n; i++) { // 2n triangular sides:
		ans.face[ans.face.length] = new Array(i, (i + 1) % n, i + n);
		ans.face[ans.face.length] = new Array(i, i + n, ((n + i - 1) % n + n));
	}
	ans.xyz = adjustXYZ(ans, 1);
	return (ans);
}

function pyramid(n) {
	var theta = 6.283185 / n; // pie angle
	var ans = new polyhedron();
	ans.name = "Y" + n;

	for (var i = 0; i < n; i++) // vertex #'s 0...n-1 around base
		ans.xyz[ans.xyz.length] = new Array(Math.cos(i * theta), Math.sin(i * theta), .2);
	ans.xyz[ans.xyz.length] = new Array(0, 0, -2); // apex

	ans.face[ans.face.length] = sequence(n - 1, 0); // base
	for (var i = 0; i < n; i++) // n triangular sides:
		ans.face[ans.face.length] = new Array(i, (i + 1) % n, n);

	ans.xyz = canonicalXYZ(ans, 3);
	return (ans);
}


//----------------polyhedron operators---------------------------
// Process: call newPoly() to clear tables
//          for each vertex of new polyhedron:
//              call newV(Vname, xyz) with symbolic name and approx location
//          for each flag of new polyhedron:
//              call newFlag(Fname, Vname1, Vname2)  with symbolic names
//          call flags2poly()  to assemble flags into polyhedron structure
//          canonicalize vertex locations
//          set name as appropriate

function kisN(poly, n) { // only kis n-sided faces, but n==0 means kiss all.
	state("Taking kis of " + (n == 0 ? "" : n + "-sided faces of ") + poly.name + "...");
	newPoly();
	for (var i = 0; i < poly.xyz.length; i++)
		newV("v" + i, poly.xyz[i]); // each old vertex is a new vertex
	var centers = faceCenters(poly); // new vertices in centers of n-sided face
	var foundAny = false; // alert if don't find any
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = "v" + poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v2 = "v" + poly.face[i][j]; // this vertex
			if (poly.face[i].length == n || n == 0) { // kiss the n's, or all
				foundAny = true; // flag that we found some
				newV("f" + i, centers[i]); // new vertex in face center
				var fname = i + v1;
				newFlag(fname, v1, v2); // three new flags, if n-sided
				newFlag(fname, v2, "f" + i);
				newFlag(fname, "f" + i, v1);
			} else
				newFlag(i, v1, v2); // same old flag, if non-n
			v1 = v2; // current becomes previous
		}
	}
	if (!foundAny)
		alert("No " + n + "-fold components were found.");
	var ans = flags2poly();
	ans.name = "k" + (n == 0 ? "" : n) + poly.name;
	ans.xyz = adjustXYZ(ans, 3); // adjust and
	//   ans.xyz = canonicalXYZ(ans, 3);            // canonicalize lightly
	return (ans);
}

function ambo(poly) { // compute ambo of argument
	state("Taking ambo of " + poly.name + "...");
	newPoly();
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = poly.face[i][poly.face[i].length - 2]; // preprevious vertex
		var v2 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (var j = 0; j < poly.face[i].length; j++) {
			var v3 = poly.face[i][j]; // this vertex
			if (v1 < v2) // new vertices at edge midpoints
				newV(midName(v1, v2), midpoint(poly.xyz[v1], poly.xyz[v2]));
			newFlag("f" + i, midName(v1, v2), midName(v2, v3)); // two new flags
			newFlag("v" + v2, midName(v2, v3), midName(v1, v2));
			v1 = v2; // shift over one
			v2 = v3;
		}
	}
	var ans = flags2poly();
	ans.name = "a" + poly.name;
	ans.xyz = adjustXYZ(ans, 2); // canonicalize lightly
	return (ans);
}

function midName(v1, v2) { // unique symbolic name, e.g. "1_2"
	if (v1 < v2)
		return (v1 + "_" + v2);
	else
		return (v2 + "_" + v1);
}

function gyro(poly) { // compute gyro of argument
	state("Taking gyro of " + poly.name + "...");
	newPoly();
	for (var i = 0; i < poly.xyz.length; i++)
		newV("v" + i, unit(poly.xyz[i])); // each old vertex is a new vertex
	var centers = faceCenters(poly); // new vertices in center of each face
	for (var i = 0; i < poly.face.length; i++)
		newV("f" + i, unit(centers[i]));
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = poly.face[i][poly.face[i].length - 2]; // preprevious vertex
		var v2 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v3 = poly.face[i][j]; // this vertex
			newV(v1 + "~" + v2, oneThird(poly.xyz[v1], poly.xyz[v2])); // new v in face
			var fname = i + "f" + v1;
			newFlag(fname, "f" + i, v1 + "~" + v2); // five new flags
			newFlag(fname, v1 + "~" + v2, v2 + "~" + v1);
			newFlag(fname, v2 + "~" + v1, "v" + v2);
			newFlag(fname, "v" + v2, v2 + "~" + v3);
			newFlag(fname, v2 + "~" + v3, "f" + i);
			v1 = v2; // shift over one
			v2 = v3;
		}
	}
	var ans = flags2poly();
	ans.name = "g" + poly.name;
	ans.xyz = adjustXYZ(ans, 3); // canonicalize lightly
	return (ans);
}

function propellor(poly) { // compute propellor of argument
	state("Taking propellor of " + poly.name + "...");
	newPoly();
	for (var i = 0; i < poly.xyz.length; i++)
		newV("v" + i, unit(poly.xyz[i])); // each old vertex is a new vertex
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = poly.face[i][poly.face[i].length - 2]; // preprevious vertex
		var v2 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v3 = poly.face[i][j]; // this vertex
			newV(v1 + "~" + v2, oneThird(poly.xyz[v1], poly.xyz[v2])); // new v in face
			var fname = i + "f" + v2;
			newFlag("v" + i, v1 + "~" + v2, v2 + "~" + v3); // five new flags
			newFlag(fname, v1 + "~" + v2, v2 + "~" + v1);
			newFlag(fname, v2 + "~" + v1, "v" + v2);
			newFlag(fname, "v" + v2, v2 + "~" + v3);
			newFlag(fname, v2 + "~" + v3, v1 + "~" + v2);
			v1 = v2; // shift over one
			v2 = v3;
		}
	}
	var ans = flags2poly();
	ans.name = "p" + poly.name;
	ans.xyz = adjustXYZ(ans, 3); // canonicalize lightly
	return (ans);
}

function reflect(poly) { // compute reflection through origin
	state("Taking reflection of " + poly.name + "...");
	for (var i = 0; i < poly.xyz.length; i++)
		poly.xyz[i] = mult(-1, poly.xyz[i]); // reflect each point
	for (var i = 0; i < poly.face.length; i++)
		poly.face[i] = poly.face[i].reverse(); // repair clockwise-ness
	poly.name = "r" + poly.name;
	poly.xyz = adjustXYZ(poly, 1); // build dual
	return (poly);
}

//--------------------------------Dual------------------------------------------
// the makeDual function computes the dual's topology, needed for canonicalization,
// where xyz's are determined.  It is then saved in a global variable globSavedDual.
// when the d operator is executed, d just returns the saved value.

function dual() { // for d operator, just swap poly with saved dual
	var ans = globSavedDual;
	globSavedDual = globSavedPoly;
	globSavedPoly = ans;
	return (ans);
}

function makeDual(poly) { // compute dual of argument, matching V and F indices
	state("Taking dual of " + poly.name + "...");
	newPoly();
	face = new Array(); // make table of face as fn of edge
	for (var i = 0; i < poly.xyz.length; i++)
		face[i] = new Object(); // create empty associative table
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v2 = poly.face[i][j]; // this vertex
			face[v1]["v" + v2] = i; // fill it.  2nd index is associative
			v1 = v2; // current becomes previous
		}
	}
	for (var i = 0; i < poly.face.length; i++) // create d's v's per p's f's
		newV(i, new Array()); // only topology needed for canonicalize
	for (var i = 0; i < poly.face.length; i++) { // one new flag for each old one
		var v1 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v2 = poly.face[i][j]; // this vertex
			newFlag(v1, face[v2]["v" + v1], i); // look up face across edge
			v1 = v2; // current becomes previous
		}
	}
	var ans = flags2poly(); // this gives one indexing of answer
	var sortF = new Array(); // but f's of dual are randomly ordered, so sort
	for (var i = 0; i < ans.face.length; i++) {
		var j = intersect(poly.face[ans.face[i][0]], poly.face[ans.face[i][1]], poly.face[ans.face[i][2]]);
		sortF[j] = ans.face[i]; // p's v for d's f is common to three of p's f's
	}
	ans.face = sortF; // replace with the sorted list of faces
	if (poly.name.substr(0, 1) != "d")
		ans.name = "d" + poly.name; // dual name is same with "d" added...
	else
		ans.name = poly.name.substr(1); // ...or removed
	return (ans);
}

//-------------------Canonicalization Algorithm--------------------------
// True canonicalization rather slow.  Using center of gravity of vertices for each
// face gives a quick "adjustment" which planarizes faces at least.

function canonicalXYZ(poly, nIterations) { // compute new vertex coords.  
	var dpoly = makeDual(poly) // v's of dual are in order or arg's f's
	state("Canonicalizing " + poly.name + "...");
	for (var count = 0; count < nIterations; count++) { // iteration:
		dpoly.xyz = reciprocalN(poly); // reciprocate face normals
		poly.xyz = reciprocalN(dpoly); // reciprocate face normals
	}
	globSavedPoly = poly; // save poly in global variable
	globSavedDual = dpoly; // save dual in global variable
	return (poly.xyz);
}

function reciprocalN(poly) { // make array of vertices reciprocal to given planes
	var ans = new Array();
	for (i = 0; i < poly.face.length; i++) { // for each face:
		var centroid = vecZero(); // running sum of vertex coords
		var normal = vecZero(); // running sum of normal vectors
		var avgEdgeDist = 0.; // running sum for avg edge distance
		var v1 = poly.face[i][poly.face[i].length - 2]; // preprevious vertex
		var v2 = poly.face[i][poly.face[i].length - 1]; // previous vertex
		for (j = 0; j < poly.face[i].length; j++) {
			var v3 = poly.face[i][j]; // this vertex
			centroid = add(centroid, poly.xyz[v3]);
			normal = add(normal, orthogonal(poly.xyz[v1], poly.xyz[v2], poly.xyz[v3]));
			avgEdgeDist = avgEdgeDist + edgeDist(poly.xyz[v1], poly.xyz[v2]);
			v1 = v2; // shift over one
			v2 = v3;
		}
		centroid = mult(1 / poly.face[i].length, centroid);
		normal = unit(normal);
		avgEdgeDist = avgEdgeDist / poly.face[i].length;
		ans[i] = reciprocal(mult(dot(centroid, normal), normal)); // based on face
		ans[i] = mult((1 + avgEdgeDist) / 2, ans[i]); // edge correction
	}
	return (ans);
}

function adjustXYZ(poly, nIterations) { // quick planarization
	var dpoly = makeDual(poly) // v's of dual are in order or arg's f's
	state("Planarizing " + poly.name + "...");
	for (var count = 0; count < nIterations; count++) { // iteration:
		dpoly.xyz = reciprocalC(poly); // reciprocate face centers
		poly.xyz = reciprocalC(dpoly); // reciprocate face centers
	}
	globSavedPoly = poly; // save poly in global variable
	globSavedDual = dpoly; // save dual in global variable
	return (poly.xyz);
}

function reciprocalC(poly) { // return array of reciprocals of face centers
	var center = faceCenters(poly);
	for (i = 0; i < poly.face.length; i++) {
		var m2 = center[i][0] * center[i][0] + center[i][1] * center[i][1] + center[i][2] * center[i][2];
		center[i][0] = center[i][0] / m2; // divide each coord by magnitude squared
		center[i][1] = center[i][1] / m2;
		center[i][2] = center[i][2] / m2;
	}
	return (center);
}

function faceCenters(poly) { // return array of "face centers"
	var ans = new Array();
	for (i = 0; i < poly.face.length; i++) {
		ans[i] = vecZero(); // running sum
		for (j = 0; j < poly.face[i].length; j++) // just average vertex coords:
			ans[i] = add(ans[i], poly.xyz[poly.face[i][j]]); // sum and...
		ans[i] = mult(1. / poly.face[i].length, ans[i]); // ...divide by n
	}
	return (ans);
}

//----------------polyhedron assembly from flags-------------------
// 4 global objects used, since javascript won't pass by reference
// property lists used as associative arrays of symbolic names

var gFlag; // gFlag[face][vertex]=next vertex of flag; symbolic triples
var gXYZ; // XYZ coordinates
var gVert; // [symbolic names] holds vertex index
var gFace; // list of symbolic names for faces

function newPoly() { // clear global vars in preparation for new construction
	gFlag = new Object();
	gXYZ = new Object();
	gVert = new Object();
	gFace = new Object();
}

function newFlag(face, v1, v2) { // add flag and face to list
	if (gFlag[face] == null)
		gFlag[face] = new Object(); // create entry for face if needed
	gFlag[face][v1] = v2; // create next-vertex entry
}

function newV(name, xyz) { // add vertex, if new, to lists
	if (gVert[name] == null) {
		gVert[name] = 0; // dummy value for now
		gXYZ[name] = xyz;
	}
}

function flags2poly() { // arrange symbolic flags into polyhedron format
	poly = new polyhedron();
	var ctr = 0; // first number the vertices
	for (var i in gVert) {
		poly.xyz[ctr] = gXYZ[i]; // and store in array
		gVert[i] = ctr;
		ctr++;
	}
	ctr = 0; // now number the faces
	for (var i in gFlag) { // for each face
		poly.face[ctr] = new Array();
		var v0; // any vertex as starting point
		for (var j in gFlag[i]) {
			v0 = gFlag[i][j];
			break; // need just one.
		}
		var v = v0; // v moves around face
		do {
			poly.face[ctr][poly.face[ctr].length] = gVert[v]; // record index
			v = gFlag[i][v]; // go to next vertex
		} while (v != v0); // until back to start
		ctr++;
	}
	newPoly(); // release memory
	poly.name = "unknown polyhedron"
	return (poly);
}

//---------------VRML output functions-----------------------

function faceColor(n, solidColor) { // color to use for n-sided faces
	switch (n) {
		case 0:
			return (solidColor); // n=0 flags solid color
		case 3:
			return ("0.9 0.3 0.3      # 3-sided faces red");
		case 4:
			return ("0.4 0.4 1.0      # 4-sided faces blue");
		case 5:
			return ("0.2 0.9 0.3      # 5-sided faces green");
		case 6:
			return ("0.9 0.9 0.2      # 6-sided faces yellow");
		case 7:
			return ("0.5 0.25 0.25    # 7-sided faces brown");
		case 8:
			return ("0.8 0.2 0.8      # 8-sided faces magenta");
		case 9:
			return ("0.5 0.2 0.8      # 9-sided faces purple");
		case 10:
			return ("0.1 0.9 0.9      # 10-sided faces grue");
		case 12:
			return ("1.0 0.6 0.1      # 12-sided faces orange");
		default:
			return ("0.5 0.5 0.5      # other faces grey");
	}
}

function outputVRML(poly) { // produce VRML polyhedron
	state("Converting to VRML format...");
	say('#VRML V1.0 ascii'); // Header stuff
	say("DEF Title Info {    # Generated by GWH's Conway-notation java script.");
	say('  string "' + poly.name + '    ' + data(poly) + '"');
	say('  }');
	say('DEF SceneInfo Info {');
	say('  string "(c) George W. Hart,  1998,   george@georgehart.com"');
	say('  string "Freely distributable for noncommercial purposes"');
	say('  }');
	say('DEF BackgroundColor Info {string "0.2 0.5 0.9"}       # blue sky');
	say('DEF Viewer Info {string "examiner"}                   # initial mode');
	say('ShapeHints {');
	say('  vertexOrdering UNKNOWN_ORDERING');
	say('  creaseAngle 0');
	say('  }');
	say('DirectionalLight {direction -.5 -1  1 intensity 0.75} # built-in lights');
	say('DirectionalLight {direction  .5  1 -1 intensity 0.75}');
	sayPoly(poly, "polyhedron", solidColor()); // the actual polyhedron
	if (compound())
		sayPoly(globSavedDual, "dual", dualColor()); // the dual polyhedron
}

function sayPoly(poly, comment, color) { // if color="", use assorted colors
	StartObj(comment);

	StartCoords();
	for (var i = 0; i < poly.xyz.length; i++)
		sayXYZ(poly.xyz[i]);
	CloseCoords();

	var lengths = new Array(); // set of face lengths
	if (color == "")
		for (var i = 0; i < poly.face.length; i++)
			lengths[poly.face[i].length] = "Y"; // flag the sizes which occur
	else
		lengths[0] = "Y"; // or flag 0 is solid color
	for (var sides = 0; sides < lengths.length; sides++)
		if (lengths[sides] == "Y") {
			StartFaces(faceColor(sides, color)); // face color
			for (var i = 0; i < poly.face.length; i++)
				if (poly.face[i].length == sides || sides == 0) // do faces
					sayFace(poly.face[i]);
			CloseFaces();
		}

	StartEdges(); // black edges
	for (var i = 0; i < poly.face.length; i++) {
		var v1 = poly.face[i][poly.face[i].length - 1]; // v1 = previous vertex
		for (var j = 0; j < poly.face[i].length; j++) {
			var v2 = poly.face[i][j] // v2 = this vertex
			if (v1 < v2) // don't duplicate
				sayEdge(v1, v2);
			v1 = v2; // current becomes previous
		}
	}
	CloseEdges();

	CloseObj();
}

function StartObj(comment) {
	say('Separator {                                           # ' + comment);
}

function CloseObj() {
	say('  }');
}

function StartCoords() {
	say('  Coordinate3 {');
	say('    point [');
}

function sayXYZ(xyz) { //write vertex coords of 3-vector
	say('\t' + digits5(xyz[0]) + " " + digits5(xyz[1]) + " " + digits5(xyz[2]) + ',');
}

function CloseCoords() {
	say('\t]');
	say('    }');
}

function StartFaces(color) { // optional color
	if (color != '') {
		say('  Material {');
		say('    diffuseColor ' + color);
		say('    }');
	}
	say('  IndexedFaceSet {');
	say('    coordIndex [');
}

function sayFace(vList) { // List given array of points  
	say0('\t');
	for (var j = 0; j < vList.length; j++)
		say0(vList[j] + ',');
	say('-1,');
}

function CloseFaces() {
	say('\t]');
	say('    }');
}

function StartEdges() {
	say('  Material {');
	say('    diffuseColor 0 0 0      # black edges');
	say('    }');
	say('  IndexedLineSet {');
	say('    coordIndex [');
}

function sayEdge(v1, v2) { // List 2 given points
	say('\t' + v1 + ',' + v2 + ',-1,');
}

function CloseEdges() {
	say('\t]');
	say('    }');
}

function say(stuff) { // add EOL
	window2.document.write(stuff + '\n')
}

function say0(stuff) { // no EOL
	window2.document.write(stuff)
}

//-----------------------math functions--------------------------

function digits5(x) {
	return (Math.round(100000. * x) / 100000.);
}

function vecZero() {
	var ans = new Array();
	ans[0] = 0.;
	ans[1] = 0.;
	ans[2] = 0.;
	return (ans);
}

function mult(c, vec) { // c times 3-vector
	var ans = new Array();
	ans[0] = c * vec[0];
	ans[1] = c * vec[1];
	ans[2] = c * vec[2];
	return (ans);
}

function add(vec1, vec2) { // sum two 3-vectors
	var ans = new Array();
	ans[0] = vec1[0] + vec2[0];
	ans[1] = vec1[1] + vec2[1];
	ans[2] = vec1[2] + vec2[2];
	return (ans);
}

function sub(vec1, vec2) { // subtract two 3-vectors
	var ans = new Array();
	ans[0] = vec1[0] - vec2[0];
	ans[1] = vec1[1] - vec2[1];
	ans[2] = vec1[2] - vec2[2];
	return (ans);
}

function dot(vec1, vec2) { // dot product two 3-vectors
	return (vec1[0] * vec2[0] + vec1[1] * vec2[1] + vec1[2] * vec2[2]);
}

function midpoint(vec1, vec2) { // mean of two 3-vectors
	var ans = new Array();
	ans[0] = 0.5 * (vec1[0] + vec2[0]);
	ans[1] = 0.5 * (vec1[1] + vec2[1]);
	ans[2] = 0.5 * (vec1[2] + vec2[2]);
	return (ans);
}

function oneThird(vec1, vec2) { // approx. (2/3)v1 + (1/3)v2   (assumes 3-vector)
	var ans = new Array();
	ans[0] = 0.7 * vec1[0] + 0.3 * vec2[0];
	ans[1] = 0.7 * vec1[1] + 0.3 * vec2[1];
	ans[2] = 0.7 * vec1[2] + 0.3 * vec2[2];
	return (ans);
}

function reciprocal(vec) { // reflect 3-vector in unit sphere
	var factor = 1. / mag2(vec);
	var ans = new Array();
	ans[0] = factor * vec[0];
	ans[1] = factor * vec[1];
	ans[2] = factor * vec[2];
	return (ans);
}

function unit(vec) { // normalize 3-vector to unit magnitude
	var size = mag2(vec);
	if (size == 0.) { // remove this test someday...
		alert("Mag(zero) -- probable bug.");
		return (vec);
	}
	var c = 1. / Math.sqrt(size);
	var ans = new Array();
	ans[0] = c * vec[0];
	ans[1] = c * vec[1];
	ans[2] = c * vec[2];
	return (ans);
}

function mag2(vec) { // magnitude squared of 3-vector
	return (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]);
}

function tangentPoint(v1, v2) { // point where line v1...v2 tangent to an origin sphere
	var d = sub(v2, v1); // difference vector
	return (sub(v1, mult(dot(d, v1) / mag2(d), d)));
}

function edgeDist(v1, v2) { // distance of line v1...v2 to origin
	return (Math.sqrt(mag2(tangentPoint(v1, v2))));
}

function orthogonal(v3, v2, v1) { // find unit vector orthog to plane of 3 pts
	var d1 = sub(v2, v1); // adjacent edge vectors
	var d2 = sub(v3, v2);
	var ans = new Array();
	ans[0] = d1[1] * d2[2] - d1[2] * d2[1]; // cross product
	ans[1] = d1[2] * d2[0] - d1[0] * d2[2];
	ans[2] = d1[0] * d2[1] - d1[1] * d2[0];
	return (ans)
}

function intersect(set1, set2, set3) { // find element common to 3 sets
	for (var i = 0; i < set1.length; i++) // by brute force search
		for (var j = 0; j < set2.length; j++)
			if (set1[i] == set2[j])
				for (var k = 0; k < set3.length; k++)
					if (set1[i] == set3[k])
						return (set1[i]);
	alert("program bug in intersect()");
	return (null);
}

function sequence(start, stop) { // make list of integers, inclusive
	var ans = new Array();
	if (start <= stop)
		for (var i = start; i <= stop; i++)
			ans[ans.length] = i;
	else
		for (var i = start; i >= stop; i--)
			ans[ans.length] = i;
	return (ans);
}

console.log("hey");

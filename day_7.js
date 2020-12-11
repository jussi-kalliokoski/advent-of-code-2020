let t="shiny gold",l=new Map(require("fs").readFileSync("/dev/stdin", "utf8").split("\n").map(l => {let[,a,,s]=l.match(/(\w+ \w+) (\w+ ){2}([^\.]+)/);return[a,s=="no other bags"?[]:s.split(", ").map(s=>{let[,c,a]=s.match(/(\d+) (\w+ \w+)/);return[parseInt(c,10),a]})];}))
console.log("first answer",[...l.values()].filter(function f(r){return r.find(a=>a[1]==t||f(l.get(a[1])))}).flat(1/0).length)
console.log("second answer:",l.get(t).map(function f(a){return a[0]+a[0]*l.get(a[1]).map(f).reduce((a,b)=>a+b,0)}).reduce((a,b)=>a+b))

import{r as t,j as e,u as f,c as b}from"./index-CYRCawG8.js";/**
 * @license lucide-react v0.344.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */var y={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor",strokeWidth:2,strokeLinecap:"round",strokeLinejoin:"round"};/**
 * @license lucide-react v0.344.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const j=s=>s.replace(/([a-z0-9])([A-Z])/g,"$1-$2").toLowerCase().trim(),i=(s,r)=>{const a=t.forwardRef(({color:l="currentColor",size:c=24,strokeWidth:o=2,absoluteStrokeWidth:d,className:m="",children:n,...x},u)=>t.createElement("svg",{ref:u,...y,width:c,height:c,stroke:l,strokeWidth:d?Number(o)*24/Number(c):o,className:["lucide",`lucide-${j(s)}`,m].join(" "),...x},[...r.map(([h,p])=>t.createElement(h,p)),...Array.isArray(n)?n:[n]]));return a.displayName=`${s}`,a};/**
 * @license lucide-react v0.344.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const v=i("Camera",[["path",{d:"M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z",key:"1tc9qg"}],["circle",{cx:"12",cy:"13",r:"3",key:"1vg3eu"}]]);/**
 * @license lucide-react v0.344.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const g=i("Gamepad2",[["line",{x1:"6",x2:"10",y1:"11",y2:"11",key:"1gktln"}],["line",{x1:"8",x2:"8",y1:"9",y2:"13",key:"qnk9ow"}],["line",{x1:"15",x2:"15.01",y1:"12",y2:"12",key:"krot7o"}],["line",{x1:"18",x2:"18.01",y1:"10",y2:"10",key:"1lcuu1"}],["path",{d:"M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z",key:"mfqc10"}]]);/**
 * @license lucide-react v0.344.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const k=i("MessageCircle",[["path",{d:"M7.9 20A9 9 0 1 0 4 16.1L2 22Z",key:"vv11sd"}]]),w=()=>e.jsx("div",{className:"bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4",children:e.jsx("div",{className:"aspect-[4/3] bg-gray-800 rounded-lg overflow-hidden relative",children:e.jsxs("div",{className:"absolute inset-0 flex items-center justify-center",children:[e.jsx(v,{className:"w-16 h-16 text-white/20"}),e.jsx("span",{className:"text-white/50 ml-4",children:"Video Capture"})]})})}),N=({message:s})=>e.jsxs(e.Fragment,{children:[e.jsxs("span",{className:"text-purple-400 font-medium",style:{color:`rgb(${s.nicknameColor[0]}, ${s.nicknameColor[1]}, ${s.nicknameColor[2]})`},children:[s.user.displayName,":"]}),e.jsx("span",{className:"text-white/90 ml-2",children:s.message})]}),C=()=>{const{messages:s}=f(),r=t.useRef(null);return t.useEffect(()=>{r.current&&(r.current.scrollTop=r.current.scrollHeight)},[s]),e.jsxs("div",{className:"bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4 h-full max-h-full flex flex-col",children:[e.jsx("div",{className:"fixed",children:e.jsxs("div",{className:"flex items-center space-x-2 mb-4",children:[e.jsx(k,{className:"w-5 h-5 text-purple-300"}),e.jsx("h3",{className:"text-white font-semibold",children:"Live Chat"})]})}),e.jsx("div",{className:"overflow-y-auto h-min min-h-full max-h-full fixed",style:{maskImage:"linear-gradient(to bottom, transparent 8%, black 40%)",maskComposite:"intersect"},ref:r,children:e.jsxs("div",{className:"overflow-y-visible hide-scrollbar space-y-2 pb-8 px-1 mr-1",children:[e.jsx("div",{className:"h-10"}),s.map((a,l)=>e.jsx("div",{className:"text-sm text-pretty break-words ",children:e.jsx(N,{message:a})},a.user+a.message+l))]})})]})},L=()=>e.jsx("div",{className:"min-h-screen max-w-screen min-w-full bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900 py-auto flex",children:e.jsxs("div",{className:"grid grid-cols-10 gap-3 content-center justify-center aspect-video flex-1 mx-3",children:[e.jsx("div",{className:"col-span-8 space-y-3 content-center",children:e.jsx("div",{className:"bg-black/40 rounded-xl backdrop-blur-sm border border-white/10 p-4",children:e.jsx("div",{className:"aspect-video bg-gray-800 rounded-lg overflow-hidden relative",children:e.jsxs("div",{className:"absolute inset-0 flex items-center justify-center",children:[e.jsx(g,{className:"w-16 h-16 text-white/20"}),e.jsx("span",{className:"text-white/50 ml-4",children:"Game Capture"})]})})})}),e.jsxs("div",{className:"col-span-2 space-y-3 flex flex-col max-w-full h-full",children:[e.jsx(C,{}),e.jsx(w,{})]})]})}),M=b("/")({component:A});function A(){return e.jsx(L,{})}export{M as Route};

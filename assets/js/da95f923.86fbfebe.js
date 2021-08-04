(self.webpackChunkdoc_ops=self.webpackChunkdoc_ops||[]).push([[332],{9473:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return c},contentTitle:function(){return l},metadata:function(){return s},toc:function(){return p},default:function(){return d}});var r=n(2122),o=n(9756),i=(n(7294),n(3905)),a=["components"],c={},l="Specification: ENGINEERING",s={unversionedId:"specs/engineering",id:"specs/engineering",isDocsHomePage:!1,title:"Specification: ENGINEERING",description:"[engineering-spec]: #engineering-spec",source:"@site/docs/specs/engineering.md",sourceDirName:"specs",slug:"/specs/engineering",permalink:"/docs/specs/engineering",editUrl:"https://github.com/iotaledger/stronghold.rs/tree/dev/documentation/docs/specs/engineering.md",version:"current",frontMatter:{},sidebar:"mySidebar",previous:{title:"Specification: SCOPE",permalink:"/docs/specs/scope"},next:{title:"Stronghold Engine Retrospective Document",permalink:"/docs/retrospective"}},p=[{value:"Frontmatter",id:"frontmatter",children:[]},{value:"Summary",id:"summary",children:[]},{value:"Logical System Design",id:"logical-system-design",children:[{value:"Low Level",id:"low-level",children:[]},{value:"High Level",id:"high-level",children:[]}]}],u={toc:p};function d(e){var t=e.components,n=(0,o.Z)(e,a);return(0,i.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"specification-engineering"},"Specification: ENGINEERING"),(0,i.kt)("h1",{id:"stronghold-engineering-specification"},"Stronghold Engineering Specification"),(0,i.kt)("h2",{id:"frontmatter"},"Frontmatter"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-yaml"},'title: Stronghold\nstub: stronghold\ndocument: Engineering Specification\nversion: 0000\nmaintainer: Daniel Thompson-Yvetot <daniel.yvetot@iota.org>\ncontributors: [Dave de Fijter <dave.defijter@iota.org>, tensorprogramming <tensordeveloper@gmail.com>, Daniel Thompson-Yvetot <daniel.yvetot@iota.org>, Marcelo Bianchi <marcelo.bianchi@iota.org>]\nsponsors: [Navin Ramachandran <navin@iota.org>]\nlicenses: ["CC-BY-INTL-3.0"]\nupdated: 2021-Apr-27\n')),(0,i.kt)("h2",{id:"summary"},"Summary"),(0,i.kt)("p",null,"This document introduces the High-Level Specification of the Stronghold."),(0,i.kt)("h2",{id:"logical-system-design"},"Logical System Design"),(0,i.kt)("h3",{id:"low-level"},"Low Level"),(0,i.kt)("p",null,"A Stronghold is composed of several interacting systems at a low level:"),(0,i.kt)("ol",null,(0,i.kt)("li",{parentName:"ol"},"Snapshot - box-encrypted file-based persistence layer"),(0,i.kt)("li",{parentName:"ol"},"Vault - a write and use protected, path-based system for storing and using secrets like private keys"),(0,i.kt)("li",{parentName:"ol"},"Store - a read/write key:value storage system for dynamic data "),(0,i.kt)("li",{parentName:"ol"},"Cache - an in-memory abstraction for vault and store"),(0,i.kt)("li",{parentName:"ol"},"Runtime - memory protection system for secrets"),(0,i.kt)("li",{parentName:"ol"},"Communication - libp2p based system for communication between strongholds")),(0,i.kt)("h3",{id:"high-level"},"High Level"),(0,i.kt)("p",null,"At the high level, Stronghold provides an official client for interfacing with a Stronghold snapshot and its records."))}d.isMDXComponent=!0},3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return p},kt:function(){return m}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=r.createContext({}),s=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},p=function(e){var t=s(e.components);return r.createElement(l.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,l=e.parentName,p=c(e,["components","mdxType","originalType","parentName"]),d=s(n),m=o,g=d["".concat(l,".").concat(m)]||d[m]||u[m]||i;return n?r.createElement(g,a(a({ref:t},p),{},{components:n})):r.createElement(g,a({ref:t},p))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=d;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c.mdxType="string"==typeof e?e:o,a[1]=c;for(var s=2;s<i;s++)a[s]=n[s];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"}}]);
<template>
		<div id="header-navbar">
			<div style="float:right">
				<a href="">Login</a> || 
				<button><a href="">Join Us</a></button>
			</div>
        </div>
</template>

<script>
    export default {
        name: "header"
        }
</script>

<style lang="scss" scoped>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300&family=JetBrains+Mono:wght@300&display=swap');

*
{
	box-sizing: border-box;
	line-height: 1.35em;
	font-family: "Inter", arial;
}

/* Scrollbar theming */
::-webkit-scrollbar
{
	width: 12px; /* For vertical scrollbar */
	height: 12px; /* For horizontal scrollbar */
}
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb
{
	background-color: #888;
	border: 2px solid transparent;
	border-radius: 8px;
	background-clip: padding-box;  
}
::-webkit-scrollbar-thumb:hover
{
	background-color: #aaa;
	border: 1px solid transparent; 
}

::-webkit-scrollbar-thumb:active { background-color: #ccc; }

html { scroll-behavior: smooth; }

body
{
	margin: 0;
	color: #eee;
	background-color: #222;
}

a
{
	color: white;
	text-decoration: none;
}
a:not(button a) { font-weight: bold; }
a:hover:not(button a:hover) { text-decoration: underline; }

button
{
	background-color: rgba(255,255,255,0.1);
	border: 1px solid rgba(255,255,255,0.5);
	border-radius: 8px;
	padding: 7px 17px;
	cursor: pointer;
	transition: 0.15s;
	font-size: 16px;
}

button:hover
{
	background-color: rgba(255,255,255,0.15);
	border-color: rgba(255,255,255,0.6);
}

</style>
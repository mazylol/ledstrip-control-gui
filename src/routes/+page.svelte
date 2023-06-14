<script lang="ts">
    import {on, off, brightness, color} from "../lib/functions";
</script>

<button on:click={on}>On</button>
<button on:click={off}>Off</button>
<br />
<button on:click={() => { brightness("low") }}>Low</button>
<button on:click={() => { brightness("medium") }}>Medium</button>
<button on:click={() => { brightness("high") }}>High</button>
<br />
<button on:click={() => { color("red") }}>Red</button>
<button on:click={() => { color("green") }}>Green</button>
<button on:click={() => { color("blue") }}>Blue</button>
<button on:click={() => { color("yellow") }}>Yellow</button>
<button on:click={() => { color("lightgreen") }}>Light Green</button>
<button on:click={() => { color("lightblue") }}>Light Blue</button>
<button on:click={() => { color("orange") }}>Orange</button>
<button on:click={() => { color("purple") }}>Purple</button>
<button on:click={() => { color("white") }}>White</button>
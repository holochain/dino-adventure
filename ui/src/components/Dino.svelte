<script lang="ts">
  import type { AuthoredDino } from "../dino_adventure/dino_adventure/types";

  import allosaurus from "../assets/dinos/allosaurus.png";
  import ankylosaurus from "../assets/dinos/ankylosaurus.png";
  import apatosaurus from "../assets/dinos/apatosaurus.png";
  import archaeopteryx from "../assets/dinos/archaeopteryx.png";
  import brachiosaurus from "../assets/dinos/brachiosaurus.png";
  import corythosaurus from "../assets/dinos/corythosaurus.png";
  import dilophosaurus from "../assets/dinos/dilophosaurus.png";
  import dimorphodon from "../assets/dinos/dimorphodon.png";
  import elasmosaurus from "../assets/dinos/elasmosaurus.png";
  import mosasaurus from "../assets/dinos/mosasaurus.png";
  import parasaurolophus from "../assets/dinos/parasaurolophus.png";
  import pteranodon from "../assets/dinos/pteranodon.png";
  import spinosaurus from "../assets/dinos/spinosaurus.png";
  import stegosaurus from "../assets/dinos/stegosaurus.png";
  import triceratops from "../assets/dinos/triceratops.png";
  import tyrannosaurusRex from "../assets/dinos/tyrannosaurus-rex.png";
  import velociraptor from "../assets/dinos/velociraptor.png";

  const lookup = {
    Allosaurus: allosaurus,
    Ankylosaurus: ankylosaurus,
    Apatosaurus: apatosaurus,
    Archaeopteryx: archaeopteryx,
    Brachiosaurus: brachiosaurus,
    Corythosaurus: corythosaurus,
    Dilophosaurus: dilophosaurus,
    Dimorphodon: dimorphodon,
    Elasmosaurus: elasmosaurus,
    Mosasaurus: mosasaurus,
    Parasaurolophus: parasaurolophus,
    Pteranodon: pteranodon,
    Spinosaurus: spinosaurus,
    Stegosaurus: stegosaurus,
    Triceratops: triceratops,
    TyrannosaurusRex: tyrannosaurusRex,
    Velociraptor: velociraptor,
  };

  let { authoredDino }: { authoredDino: AuthoredDino } = $props();
</script>

<img
  alt={authoredDino.dino.name}
  class="w-16"
  src={lookup[authoredDino.dino.dino_kind.type]}
/>
<p>{authoredDino.dino.name}</p>

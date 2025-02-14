import type {AuthoredDino, Dino, DinoKind} from "../dino_adventure/dino_adventure/types";
import {callZome, signalHandler} from "./common.svelte";
import type {SignedActionHashed} from "@holochain/client";

let dinoState: AuthoredDino[] = $state([]);

let dinosFirstLoad = $state(false);

export const getDinoState = () => dinoState;

export const getDinosFirstLoaded = () => dinosFirstLoad;

export const createDino = async (dino: Dino): Promise<AuthoredDino> => {
    return callZome({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "create_dino",
        cap_secret: null,
        payload: dino,
    })
}

export const getAllDinos = async (): Promise<AuthoredDino[]> => {
    const authoredDinos = await callZome<AuthoredDino[]>({
        role_name: "dino_adventure",
        zome_name: "dino_adventure",
        fn_name: "get_all_dinos",
        cap_secret: null,
        payload: null,
    });

    dinoState = [
        // Remove any content that is no longer in the authored dino list
        ...dinoState.filter((authoredDino) => authoredDinos.find((dino) => dino.dino.name == authoredDino.dino.name)),
        // Then add new content
        ...authoredDinos
    ];
    setTimeout(() => {
        dinosFirstLoad = true;
    }, 1000)


    return authoredDinos;
}

(async () => {
    signalHandler.addSignalHandler("dino_adventure:EntryCreated:Dino", (dino: Dino, action: SignedActionHashed) => {
        dinoState = dinoState.filter((authoredDino) => authoredDino.dino.name != dino.name);

        dinoState.push({
            dino: dino,
            address: action.hashed.hash,
            author: action.hashed.content.author,
        });
    });

    // Load all the dinos when the page loads
    await getAllDinos();
})();

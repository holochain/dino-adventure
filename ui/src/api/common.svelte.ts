import {AppWebsocket, HolochainError, type AppCallZomeRequest} from "@holochain/client";

let client: AppWebsocket | null = null;

let isConnected = $state(false);

export const getIsConnected = () => isConnected;

export const setIsConnected = (value: boolean) => isConnected = value;

export const callZome = async <T>(request: AppCallZomeRequest): Promise<T> => {
    if (!client) {
        client = await AppWebsocket.connect();
    }

    isConnected = true;

    try {
        return (await client.callZome(request)) as T
    } catch (e) {
        if (e instanceof HolochainError) {
            if (e.name == 'ConnectionError') {
                // Clear the client so that it will be reconnected on the next call
                client = null;
                isConnected = false;
            }
        }

        throw e;
    }
}

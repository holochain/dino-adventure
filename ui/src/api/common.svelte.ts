import {AppWebsocket, HolochainError, type AppCallZomeRequest} from "@holochain/client";

let client: AppWebsocket | null = null;

let isConnected = $state(false);

export const getIsConnected = () => isConnected;

export const callZome = async <T>(request: AppCallZomeRequest): Promise<T> => {
    console.log('Calling zome', request);
    console.log('connected?', isConnected);
    console.log('client', client);
    if (!client) {
        console.log('reconnecting');
        client = await AppWebsocket.connect();
        console.log('reconnected');
    }

    isConnected = true;
    console.log('now connected?', isConnected);

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

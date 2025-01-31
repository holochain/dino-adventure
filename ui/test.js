import {AdminWebsocket, AppWebsocket, HolochainError} from '@holochain/client';

(async () => {
  const admin = await AdminWebsocket.connect({
    url: 'ws://localhost:43911',
    defaultTimeout: 30000,
    wsClientOptions: {
      origin: 'http://localhost:8888',
    }
  });

  const tokenResponse = await admin.issueAppAuthenticationToken({
    installed_app_id: 'DinoAdventure',
    expiry_seconds: 30,
    single_use: true,
  });

  const appInterface = await admin.attachAppInterface({
    port: 0,
    allowed_origins: "*",
  });

  let client = await AppWebsocket.connect({
    url: `ws://localhost:${appInterface.port}`,
    token: tokenResponse.token,
    wsClientOptions: {
      origin: "http://localhost:8888",
    }
  });

  while (true) {
    let appInfo;
    let keepGoing = false;
    try {
      appInfo = await client.appInfo()
    } catch (e) {
      if (e instanceof HolochainError) {
        if (e.name === 'ConnectionError') {
          console.log('Connection error, retrying...')

          keepGoing = true;
          try {
            const tokenResponse = await admin.issueAppAuthenticationToken({
              installed_app_id: 'DinoAdventure',
              expiry_seconds: 30,
              single_use: true,
            });

            const appInterface = await admin.attachAppInterface({
              port: 0,
              allowed_origins: "*",
            });

            client = await AppWebsocket.connect({
              url: `ws://localhost:${appInterface.port}`,
              token: tokenResponse.token,
              wsClientOptions: {
                origin: "http://localhost:8888",
              }
            });
          } catch (_e) {

          }
        }
      }

      if (!keepGoing) {
        console.log(e)
        break;
      }
    }

    console.log(appInfo)

    await new Promise(resolve => setTimeout(resolve, 1000));
  }
})();

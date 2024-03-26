import freighterApi from "@stellar/freighter-api";
import {
    isConnected,
    getPublicKey,
    signAuthEntry,
    signTransaction,
    signBlob,
  } from "@stellar/freighter-api";

  import { isAllowed } from "@stellar/freighter-api";

if (await isAllowed()) {
  alert("User has allowed your app!");
}
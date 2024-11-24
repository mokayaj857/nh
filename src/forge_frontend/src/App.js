import React, { useState, useEffect } from "react";
import { HttpAgent, Actor } from "@dfinity/agent";
import { idlFactory } from "./declarations/your_contract";
import { AuthClient } from "@dfinity/auth-client";

const StakingDapp = () => {
  const [network, setNetwork] = useState("Testnet");
  const [address, setAddress] = useState("");
  const [publicKey, setPublicKey] = useState(null);
  const [balance, setBalance] = useState(null);
  const [utxos, setUtxos] = useState([]);
  const [fee, setFee] = useState(null);
  const [transaction, setTransaction] = useState(null);
  const [sendRequest, setSendRequest] = useState({
    destination_address: "",
    amount_in_satoshi: 0,
    fee: 0,
  });

  const agent = new HttpAgent({ host: "https://ic0.app" });
  const contractActor = Actor.createActor(idlFactory, {
    agent,
    canisterId: "your-canister-id",
  });

  const fetchBitcoinAddress = async () => {
    try {
      const result = await contractActor.get_address(network, "test_key_1", []);
      setAddress(result);
    } catch (error) {
      console.error("Error fetching address:", error);
    }
  };

  const fetchECDSAPublicKey = async () => {
    try {
      const result = await contractActor.get_ecdsa_public_key(
        "test_key_1",
        []
      );
      setPublicKey(result);
    } catch (error) {
      console.error("Error fetching ECDSA public key:", error);
    }
  };

  const fetchBalance = async () => {
    try {
      const result = await contractActor.get_balance(address);
      setBalance(result);
    } catch (error) {
      console.error("Error fetching balance:", error);
    }
  };

  const fetchUTXOs = async () => {
    try {
      const result = await contractActor.get_utxos(network, address);
      setUtxos(result);
    } catch (error) {
      console.error("Error fetching UTXOs:", error);
    }
  };

  const fetchFee = async () => {
    try {
      const result = await contractActor.get_fee_per_byte(network);
      setFee(result);
    } catch (error) {
      console.error("Error fetching fee:", error);
    }
  };

  const buildTransaction = async () => {
    try {
      const { destination_address, amount_in_satoshi, fee } = sendRequest;
      const result = await contractActor.build_transaction_with_fee(
        utxos,
        address,
        destination_address,
        amount_in_satoshi,
        fee
      );
      setTransaction(result);
    } catch (error) {
      console.error("Error building transaction:", error);
    }
  };

  const sendTransaction = async () => {
    try {
      await contractActor.send_transaction(network, transaction);
      alert("Transaction sent successfully!");
    } catch (error) {
      console.error("Error sending transaction:", error);
    }
  };

  return (
    <div style={{ padding: "20px" }}>
      <h1>ICP Bitcoin Wallet</h1>

      <section>
        <h2>Network Selection</h2>
        <select
          value={network}
          onChange={(e) => setNetwork(e.target.value)}
        >
          <option value="Mainnet">Mainnet</option>
          <option value="Testnet">Testnet</option>
          <option value="Regtest">Regtest</option>
        </select>
      </section>

      <section>
        <h2>Bitcoin Address</h2>
        <button onClick={fetchBitcoinAddress}>Fetch Address</button>
        {address && <p>Address: {address}</p>}
      </section>

      <section>
        <h2>ECDSA Public Key</h2>
        <button onClick={fetchECDSAPublicKey}>Fetch Public Key</button>
        {publicKey && <pre>{JSON.stringify(publicKey, null, 2)}</pre>}
      </section>

      <section>
        <h2>Balance</h2>
        <button onClick={fetchBalance}>Fetch Balance</button>
        {balance !== null && <p>Balance: {balance} satoshi</p>}
      </section>

      <section>
        <h2>UTXOs</h2>
        <button onClick={fetchUTXOs}>Fetch UTXOs</button>
        {utxos.length > 0 && (
          <pre>{JSON.stringify(utxos, null, 2)}</pre>
        )}
      </section>

      <section>
        <h2>Fee Estimation</h2>
        <button onClick={fetchFee}>Fetch Fee</button>
        {fee && <p>Fee per byte: {fee} millisatoshi</p>}
      </section>

      <section>
        <h2>Send Bitcoin</h2>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            buildTransaction();
          }}
        >
          <label>
            Destination Address:
            <input
              type="text"
              value={sendRequest.destination_address}
              onChange={(e) =>
                setSendRequest({
                  ...sendRequest,
                  destination_address: e.target.value,
                })
              }
            />
          </label>
          <br />
          <label>
            Amount (satoshi):
            <input
              type="number"
              value={sendRequest.amount_in_satoshi}
              onChange={(e) =>
                setSendRequest({
                  ...sendRequest,
                  amount_in_satoshi: parseInt(e.target.value, 10),
                })
              }
            />
          </label>
          <br />
          <label>
            Fee (satoshi):
            <input
              type="number"
              value={sendRequest.fee}
              onChange={(e) =>
                setSendRequest({
                  ...sendRequest,
                  fee: parseInt(e.target.value, 10),
                })
              }
            />
          </label>
          <br />
          <button type="submit">Build Transaction</button>
        </form>
        {transaction && (
          <>
            <pre>{JSON.stringify(transaction, null, 2)}</pre>
            <button onClick={sendTransaction}>Send Transaction</button>
          </>
        )}
      </section>
    </div>
  );
};

export default StakingDapp;

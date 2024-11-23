import React from 'react';
import classes from './AdminTesting.module.css';

const AdminTesting = (props) => {
  return (
    <div className={classes.for_testing}>
      <p>FOR TESTING PURPOSE ONLY</p>
      <button onClick={props.claimBTC}>Claim for 1000 BTC (User)</button>
      &nbsp; &nbsp;
      <button onClick={props.redistributeRewards}>
        {props.page === 1
          ? `Redistribute rewards (Admin)`
          : `Custom redistribution (Admin)`}
      </button>
      <div className={classes.network}>
        <p>
          Selected Network: <b>{props.network.name}</b>
          &nbsp; id: <b>{props.network.id}</b>
        </p>
        <p>Contract Balance: {props.contractBalance} CKBTC (BTC) </p>
        <p>Staking Contract address: {props.tokenStakingContract._address}</p>
      </div>
    </div>
  );
};

export default AdminTesting;

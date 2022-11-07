# crop_insurance_smart_contract

This is a Rust Smart Contract(running on Solana Blockchain), it is expected to integrate to oracle protocol(eg chainlink) to receive accurate weather data.
The weather data will be processed by the smart contract to determine if rainfall failed during the planting month, if this happens
then insured small scale farmers will automatically be compensated by the crediting of the farmers' mobile money wallet.

There will also be integration to Safaricom M-Pesa Mobile Money Payment Gateway (i.e exposed API endpoints for accessing M-Pesa services by Kenyan Telco called "Safaricom"). 
The Kenyan Telco "Safaricom" has provided M-Pesa API endpoints for B2C, C2B and B2B (https://developer.safaricom.co.ke/Documentation).
# crop_insurance_smart_contract

This is Rust Smart Contract(running on Solana Blockchain), it is expected to integrate to oracle protocol(eg chainlink) to receive accurate weather data.
The weather data will be processed by the smart contract to determine if rainfall failed during the planting season, if this happens
then insured small scale farmers will automatically be compensated. The compensatation is done by the crediting of the farmers mobile money.
A RESTful Rust(ActixWeb) Web Service that connects to PostgreSQL database will be developed, it will integrate with Safaricom M-Pesa Mobile Money Payment Gateway (i.e exposed API endpoints for accessing M-Pesa services by Kenyan Telco called "Safaricom") and enables customers to pay for utilities like water, PayTv, electricity from their phone wallets. The Kenyan Telco "Safaricom" has provided M-Pesa API endpoints for B2C, C2B and B2B (https://developer.safaricom.co.ke/Documentation). 
For this project, the Rust(ActixWeb) Web Service will support B2C "Business to Customer", other features will be added soon.
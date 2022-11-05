// No imports needed: web3, anchor, pg and more are globally available

describe('Test', async() => {
  //const provider = anchor.AnchorProvider.env();
  //anchor.setProvider(provider);
  //const program = anchor.workspace.CertBank as Program<CertBank>;
  //const data = new BN(12345678);
  //const data: number = 1;
  const day: number = 1;
  const month: number = 11;
  const year: number = 2022;
  const weather: string = "plentyrain";

  const day2: number = 3;
  const weather2: string = "poorrain";//norain

  it('create_crop_info!', async () => {
    const [cropInfoPDA, _] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("crop-info"),
          pg.wallet.publicKey.toBuffer()
        ],
        pg.program.programId
      );
    /*
    console.log("On-chain step 1");
    await pg.program.methods
      .createCropInfo(month,year)
      .accounts({
        signer: pg.wallet.publicKey,
        cropInfo: cropInfoPDA,
      })
      .rpc();
    console.log("On-chain step 2");
    //expect((await pg.program.account.certAccount.fetch(certAccountPDA)).data).to.equal("brian");
    // Fetch the created account
    const certVault = await pg.program.account.cropInfo.fetch(cropInfoPDA);

    console.log("On-chain count is:", certVault.count);
    //console.log("On-chain data is:", certVault.month.toString());
    //console.log("On-chain data is:", certVault.weather.toString());
    console.log("On-chain bump is:", certVault.bump);

    // Check whether the data on-chain is equal to local 'data'
    //assert(data == certVault.idNo);
    */
    /*
    await pg.program.methods
      .addCropInfo(day2,month,weather2)
      .accounts({
        signer: pg.wallet.publicKey,
        cropInfo: cropInfoPDA,
      })
      .rpc();

    const certVault2 = await pg.program.account.cropInfo.fetch(cropInfoPDA);

    console.log("On-chain count is:", certVault2.count);
    console.log("On-chain data is:", certVault2.month.toString());
    //console.log("On-chain data is:", certVault2.weather.toString());
    //console.log("On-chain data is:", certVault2.certAccount.toString());
    //console.log("On-chain data is:", certAccount.data.toString());
    //console.log("On-chain certAccount item count is:", certVault2.certAccount.toString().length);
    //console.log("On-chain certAccount items is:", certVault2.certAccount.indexOf("1"));
    console.log("On-chain bump is:", certVault2.bump);
    */
    // Check whether the data on-chain is equal to local 'data'
    //assert(data == certAccount.idNo);
    let listener = null;

    let [event, slot] = await new Promise((resolve, _reject) => {
      listener = pg.program.addEventListener("MyEvent", (event, slot) => {
        resolve([event, slot]);
      });
      //pg.program.rpc.addCropInfo();
      pg.program.methods
      .addCropInfo(day2,month,year,weather2)
      .accounts({
        signer: pg.wallet.publicKey,
        cropInfo: cropInfoPDA,
      })
      .rpc();
    });
    await pg.program.removeEventListener(listener);

    //assert.isAbove(slot, 0);
    //assert.strictEqual(event.data.toNumber(), 5);
    //assert.strictEqual(event.label, "hello");
    //console.log("On-chain event.data is:", event.data.toNumber());
    console.log("On-chain event.data is:", event.data);
    console.log("On-chain event.label is:", event.label);
    let isProcessed = event.data
    console.log("On-chain isProcessed is:", isProcessed);
  });
});
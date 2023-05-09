import { expect, describeSuite, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { BN } from "@polkadot/util";
describeSuite({
  id: "ZTN",
  title: "Zombie Tanssi Test",
  foundationMethods: "zombie",
  testCases: function ({ it, context, log }) {
    let paraApi: ApiPromise;
    let relayApi: ApiPromise;
    let container2000Api: ApiPromise;
    let container2001Api: ApiPromise;

    beforeAll(async () => {
      
      paraApi = context.polkadotJs({ apiName: "Tanssi" });
      relayApi = context.polkadotJs({ apiName: "Relay" });
      container2000Api = context.polkadotJs({ apiName: "Container2000" });
      container2001Api = context.polkadotJs({ apiName: "Container2001" });

      const relayNetwork = relayApi.consts.system.version.specName.toString();
      expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

      const paraNetwork = paraApi.consts.system.version.specName.toString();
      expect(paraNetwork, "Para API incorrect").to.contain("orchestrator-template-parachain");

      const container2000Network = container2000Api.consts.system.version.specName.toString();
      expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");

      const container2001Network = container2001Api.consts.system.version.specName.toString();
      expect(container2001Network, "Container2001 API incorrect").to.contain("container-chain-template");

    }, 120000);

    it({
      id: "T01",
      title: "Blocks are being produced on parachain",
      test: async function () {
        const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T02",
      title: "Blocks are being produced on container 2000",
      test: async function () {
        const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T03",
      title: "Blocks are being produced on container 2001",
      test: async function () {
        const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });
 
    it({
      id: "T04",
      title: "Test assignation is correct",
      test: async function () {
        const tanssiCollators = (await paraApi.query.collatorAssignment.collatorContainerChain()).orchestratorChain.map((v): string =>
        v.toString()
        );
        const authorities = (await paraApi.query.aura.authorities());

        let getKeyOwnersFromAuthorities = [];

        for (var authority of authorities) {
          const owner = (await paraApi.query.session.keyOwner([
            "aura",
             authority
          ]
          ));
          getKeyOwnersFromAuthorities.push(owner.toString());
        }

        for (let i = 0; i < tanssiCollators.length; i++) {
          expect(tanssiCollators[i]).to.be.equal(getKeyOwnersFromAuthorities[i]);
        }
      },
    });

    it({
      id: "T05",
      title: "Test container chain 2000 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toHuman()[paraId];

        const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toHuman();

        for (let i = 0; i < containerChainCollators.length; i++) {
          expect(containerChainCollators[i]).to.be.equal(writtenCollators[i]);
        }
      },
    });

    it({
      id: "T06",
      title: "Test container chain 2001 assignation is correct",
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();

        const containerChainCollators = assignment.containerChains.toHuman()[paraId];

        const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toHuman();

        for (let i = 0; i < containerChainCollators.length; i++) {
          expect(containerChainCollators[i]).to.be.equal(writtenCollators[i]);
        }
      },
    });

    it({
      id: "T07",
      title: "Test author noting is correct for both containers",
      timeout: 60000,
      test: async function () {
        const assignment = (await paraApi.query.collatorAssignment.collatorContainerChain());
        const paraId2000 = (await container2000Api.query.parachainInfo.parachainId());
        const paraId2001 = (await container2001Api.query.parachainInfo.parachainId());

        const containerChainCollators2000 = assignment.containerChains.toHuman()[paraId2000.toString()];
        const containerChainCollators2001 = assignment.containerChains.toHuman()[paraId2001.toString()];

        await context.waitBlock(3, "Tanssi");
        const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);
        const author2001 = await paraApi.query.authorNoting.latestAuthor(paraId2001);

        expect(containerChainCollators2000.includes(author2000.toString())).to.be.true;
        expect(containerChainCollators2001.includes(author2001.toString())).to.be.true;
      },
    });


    it({
      id: "T08",
      title: "Test live registration of container chain 2002",
      timeout: 1200000,
      test: async function () {
        const keyring = new Keyring({ type: 'sr25519' });
        let alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
        const emptyGenesisData = () => {
            // TODO: fill with default value for all the entries of ContainerChainGenesisData
            let g = {
              id: "container-chain-2002",
              name: "Container Chain 2002",
            };
            return g;
        };
        const containerChainGenesisData = emptyGenesisData();

        const tx = paraApi.tx.registrar.register(2002, containerChainGenesisData);
        await paraApi.tx.sudo.sudo(tx).signAndSend(alice);
        
        // TODO: we need to manually register the parachain in the relay as well
        await context.waitBlock(8, "Tanssi");

        const registered = (await paraApi.query.registrar.registeredParaIds());
        //console.log("regitrar: ", registered);

	// This ws api is only available after the node detects its assignment
	const wsProvider = new WsProvider('ws://127.0.0.1:9951');
        let container2002Api = await ApiPromise.create({ provider: wsProvider });
        console.log(api.genesisHash.toHex());

        const container2002Network = container2002Api.consts.system.version.specName.toString();
        expect(container2002Network, "Container2002 API incorrect").to.contain("container-chain-template");

        expect(false).to.be.true;
      },
    });

  },
});

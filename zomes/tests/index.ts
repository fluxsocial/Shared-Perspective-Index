import { Orchestrator, Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import { HoloHash, InstallAppRequest } from '@holochain/conductor-api'
import path from 'path'

const network = {
    transport_pool: [{
      type: TransportConfigType.Proxy,
      sub_transport: {type: TransportConfigType.Quic},
      proxy_config: {
        type: ProxyConfigType.LocalProxyServer,
        proxy_accept_config: ProxyAcceptConfig.AcceptAll
      }
    }],
    bootstrap_service: "https://bootstrap.holo.host"
};
//const conductorConfig = Config.gen({network});
const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const sp = path.join(__dirname, '../../workdir/shared-perspectives.dna')

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [sp] // contains 1 dna
  ]
]

const orchestrator = new Orchestrator()

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

orchestrator.registerScenario("create and get public expression", async (s, t) => {
  const [alice] = await s.players([conductorConfig])
  const [[alice_happ]] = await alice.installAgentsHapps(installation)

  const indexSharedPerspective = await alice_happ.cells[0].call("shared_perspective_index", "index_shared_perspective", 
    {
      key: "someuuid", 
      sharedPerspective: {
        name: "My group", description: "Alice's group", type: "Holochain", linkLanguages: ["langhash://exprurl"],
        allowedExpressionLanguages: ["langhash://exprurl"],
        requiredExpressionLanguages: ["langhash://exprurl"],
        author: {
          did: "did:junto:alice"
        },
        timestamp: new Date().toISOString(),
        proof: {
          signature: "sig",
          key: "key"
        }
      }
    })
  console.log("Index", indexSharedPerspective);
  t.ok(indexSharedPerspective);

  //Try and get the sp by key
  const getSp = await alice_happ.cells[0].call("shared_perspective_index", "get_latest_shared_perspective", "someuuid")
  console.log("Got latest shared perspective", getSp);
  t.ok(getSp);

  const indexSharedPerspective2 = await alice_happ.cells[0].call("shared_perspective_index", "index_shared_perspective", 
  {
    key: "someuuid", 
    sharedPerspective: {
      name: "My group 2", description: "Alice's group 2", type: "Holochain", linkLanguages: ["langhash://exprurl"],
      allowedExpressionLanguages: ["langhash://exprurl"],
      requiredExpressionLanguages: ["langhash://exprurl"],
      author: {
        did: "did:junto:alice"
      },
      timestamp: new Date().toISOString(),
      proof: {
        signature: "sig",
        key: "key"
      }
    }
  })
  console.log("Index 2", indexSharedPerspective2);
  t.ok(indexSharedPerspective2);

  //Try and get the sp by key
  const getSpAll = await alice_happ.cells[0].call("shared_perspective_index", "get_all_shared_perspectives", "someuuid")
  console.log("Got latest shared perspective", getSpAll);
  t.ok(getSpAll);
  t.assert(getSpAll.length, 2);
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)
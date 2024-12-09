---
title: Questions and ideas
description: Questions and ideas about Kittynode.
---

This page is a collection of questions and ideas we have about Kittynode. We are building and thinking in the open.

## Questions

### How do we solve the "pump the gas coordination problem"?

This is a problem we noticed during the "pump the gas" initiative to increase the block gas limit on Ethereum. The general problem is that the Ethereum community cannot necessarily coordinate an increase in the block gas limit, even if a majority of the community votes for it. What we mean is that this is really gatekept by large institutional stakers (like Coinbase), as well as the core devs who decide on the default parameters in their clients. Orbit SSF should be able to democratize votes further, but this still feels like an issue. We should start by doing some more research and quantifying the problem.

### Should every internet connected device run an Ethereum node?

To have trustless access to the internet, it will be necessary unless they are bootstrapping with weak subjectivity endpoints. The current answer is yes, but it's unknown how many light clients vs stateless clients and we ought to do more research (there was a light clients talk at Devcon Thailand about this).

### What kind of node infrastructure do we support?

We expect operators to aggregate their validators, but potentially diversify and run multiple nodes for robustness. We do support these workflows, as it's simpler. However highly specialized use cases (like competitive block builders), is not something that makes sense for us to support due to the nature of it.

### Should voting in consensus require self-operation of a node, or can that part be delegated?

Current answer: We think that self-operation of a node is a must for maintaining self-sovereignty. There is no way to enforce these things in protocol, as it relates to the fork of the chain which is being run by the client.

### Do machines have the same voting rights as humans; and if so, does it open additional attack vectors?

Current answer: Seemingly yes, with some caveats. This has credible neutrality implications, which is always difficult to be completely credibly neutral. However most chains seek to be credibly neutral while minimizing bad things for all the users (like toxic MEV).

## Ideas

### The internet stack of the future

1. Local WiFi routers and local modems no longer exist, and internet is provided via space satellites and terrestrial towers.
2. Internet is always on and you are always connected (99.999% or higher).
3. Data bandwidth could exceed 100 Gbps, and latency could be under 1 ms.
4. Centralized root name servers are replaced by decentralized DNS.

### The core dev problem

One problem in voting in networks is that you usually have to run a client that is from a group of client developers. One thing that concerns us is that it means a small group of people control the software that everyone is running.

This may be a weird or inaccurate example, but let's list it out anyways. Let's say the broad Ethereum community wants to signal a gas limit change. Let's assume that quantifiably 95% of the community desires this change. The community decides that a great way to make this into a reality is to change the default parameters for the clients. However the community cannot create that change, even though they represent a majority. The clients are controlled by smaller group of people. While the community could fork the client, this fork would not gain security updates such as the main client.

This all seems to stem from the fact that 95% of the community is not 95% of the validators. If it were, then all those 95% of validators could easily make the change through voting in their Kittynode. If the rest of the core devs disagree, then, well, I'm not sure what would happen. Have to think about it some more.

However this does create another north star for Kittynode: we want the community size (users) and validator set to look really similar.

### The impressionable dumb user problem

This is a classic human problem. The basic idea is that sometimes people don't know what's best for them. How does this play into consensus and voting? What if a large majority of dumb users can be manipulated into voting for something that they seemingly want but is against their interests?

Separation of power can help here, but it's still tricky because the rules on separation of power should be flexible over time by the users.

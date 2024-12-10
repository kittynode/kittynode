---
title: Questions 
description: Questions about Kittynode's design.
---

This page is a collection of questions we have about Kittynode. We are building and thinking out loud here.

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

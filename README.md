# Cosmwasm-Bridge

This repo contain necessary components for retrieving the verified data from Bandchain.
The scenario here is that you have a contract called the consumer, which requires the data from Band's oracle.
You will need the five following components.
1. A datasource on Bandchain (See more details here 👉 [ds](/ds))
2. An oracle script on Bandchain (See more details here 👉 [os](/os))
3. Request & relayer service which is an off-chain part (See more details here 👉 [example_interaction_js](/example_interaction_js))
4. Bridge contract on Terra chain (See more details here 👉 [bridge](/bridge))
5. Consumer contract on Terra chain (See more details here 👉 [simple_consumer](/simple_consumer))

### This is the overall image that demonstrate the flow of retrieving the verified data from Bandchain and then save to the Consumer's state.

![img](https://user-images.githubusercontent.com/12705423/157718539-38f5e1c9-1f19-4d3f-aeac-c4f161808cd1.png)

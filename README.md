# Cosmwasm-Bridge

This repo contain necessary components for retrieving the verified data from Bandchain.
The scenario here is that you have a contract called the consumer, which requires the data from Band's oracle.
You will need the five following components.
1. A datasource on Bandchain (See more details here 👉 [ds](/ds))
2. An oracle script on Bandchain (See more details here 👉 [os](/os))
3. Request & relayer service which is an off-chain part (See more details here 👉 [example_interaction_js](/example_interaction_js))
4. Bridge contract on Terra chain (See more details here 👉 [bridge](/bridge))
5. Consumer contract on Terra chain (See more details here 👉 [simple_consumer](/simple_consumer))

### This image below demonstrates the overall flow of retrieving the verified data from Bandchain and then saving it to the Consumer's state.

![img](https://user-images.githubusercontent.com/12705423/157853134-f6d52586-9875-4ca4-8759-36258d7cb2c8.png)

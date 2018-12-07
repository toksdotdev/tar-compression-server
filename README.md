# Scaling Python Microservices with Rust - (A .tar Compression Server)

If you have previously been in a scenerio where you've written web applications that require processing of heavy task such as compressing a file on the fly etc.

### The problem

Have you been in a scenerio where both the front-end and backend logic for different business operations are all coupled together? The problem with such an approach is that as the application logic grows, and as more business oriented features are integrated, it makes it hard to maintain, unit test functionalities effectively, and more importantly scale such an applicatopm in cases of huge traffic.

As such a problem persists, this can cost little to several thousands of dollars to the organisation. Such ckind of approach are called **monolith**.

### The solution

In order to solve this challenge, it requires that we utilize a system that is decoupled, where application operations can be grouped as a services, and then exists seperately. In such, such an service can still operate even if other other grouped services crash. To solve this, we use a concept called **microservices**.

## What exactly are microservices?

A Microservice is a software development technique—a variant of the service-oriented architecture (SOA) architectural style that structures an application as a collection of loosely coupled services.

Services in a microservice architecture (MSA) are often processes that communicate over a network to fulfill a goal using technology-agnostic protocols such as **HTTP**.

![image.png](attachment:image.png)

## Characteristics of Microservices

- Small in size,
- Messaging enabled,
- Bounded by contexts,
- Autonomously developed,
- Independently deployable,
- Decentralized
- Built and released with automated processes
- Independent test for each services.

## Project

- **USE THE NIGHTLY COMPILER OF RUST, AS ROCKET ONLY SUPPORTS `nightly` CURRENTLY**
- **THE CODE ISN'T COMPLETE YET**

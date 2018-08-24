describe('starting soon', () => {
  it('should load', () => {
    cy.visit('http://localhost:3001')

    cy.get('h1')
      .contains('Starting soon')
  })
})

describe('break', () => {
  it('should load', () => {
    cy.visit('http://localhost:3001/break')

    cy.get('h1')
      .contains('Quick Break')
  })
})

describe('ending', () => {
  it('should load', () => {
    cy.visit('http://localhost:3001/ending')

    cy.get('h1')
      .contains('Thanks for watching!')
  })
})
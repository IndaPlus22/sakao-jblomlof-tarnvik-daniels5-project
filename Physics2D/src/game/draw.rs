fn draw(&mut self, event: &Event, window: &mut PistonWindow) {

    // Update application window.
    window.draw_2d(event, |context, graphics, _| {
        // Fill the window with white colour.
        piston_window::clear(BLACK_COLOUR, graphics);

        for i in 0..self.movers.len() {
            self.movers[i].render(graphics, context.transform);
        }
    });
}
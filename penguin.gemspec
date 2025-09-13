# frozen_string_literal: true

require_relative "lib/penguin/version"

Gem::Specification.new do |spec|
  spec.name = "penguin"
  spec.version = Penguin::VERSION
  spec.authors = ["Nick Spain"]
  spec.email = ["nicholas.spain@stileeducation.com"]
  spec.summary = "Generate and parse BSON Object IDs"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.2.0"
  spec.required_rubygems_version = ">= 3.3.11"


  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ Gemfile .gitignore test/])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/penguin_object_id/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9.91"
end
